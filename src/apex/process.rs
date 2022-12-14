pub mod basic {
    use crate::bindings::*;

    pub type ProcessName = ApexName;
    pub type ProcessIndex = ApexInteger;
    pub type StackSize = ApexUnsigned;
    pub type WaitingRange = ApexInteger;

    pub type Priority = ApexInteger;
    // type PriorityType = ApexInteger;
    // #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    // #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    // pub struct Priority(PriorityType);
    // pub const MIN_PRIORITY_VALUE: PriorityType = 1;
    // pub const MAX_PRIORITY_VALUE: PriorityType = 239;

    // impl TryFrom<PriorityType> for Priority {
    //     type Error = PriorityType;

    //     fn try_from(value: PriorityType) -> Result<Self, Self::Error> {
    //         if let MIN_PRIORITY_VALUE..=MAX_PRIORITY_VALUE = value {
    //             return Ok(Priority(value));
    //         }
    //         Err(value)
    //     }
    // }

    // impl From<Priority> for PriorityType {
    //     fn from(prio: Priority) -> Self {
    //         prio.0
    //     }
    // }

    // #[cfg(feature = "serde")]
    // impl<'de> serde::Deserialize<'de> for Priority {
    //     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    //     where
    //         D: serde::Deserializer<'de>,
    //     {
    //         let prio: PriorityType = serde::Deserialize::deserialize(deserializer)?;
    //         prio.try_into().map_err(serde::de::Error::custom)
    //     }
    // }

    // impl Default for Priority {
    //     fn default() -> Self {
    //         Priority(MIN_PRIORITY_VALUE)
    //     }
    // }

    pub type LockLevel = ApexInteger;
    // type LockLevelType = ApexInteger;
    // #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    // #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    // pub struct LockLevel(LockLevelType);
    // pub const MIN_LOCK_LEVEL: LockLevelType = 0;
    // pub const MAX_LOCK_LEVEL: LockLevelType = 16;

    // impl TryFrom<LockLevelType> for LockLevel {
    //     type Error = LockLevelType;

    //     fn try_from(value: LockLevelType) -> Result<Self, Self::Error> {
    //         if let MIN_LOCK_LEVEL..=MAX_LOCK_LEVEL = value {
    //             return Ok(LockLevel(value));
    //         }
    //         Err(value)
    //     }
    // }

    // impl From<LockLevel> for LockLevelType {
    //     fn from(lock: LockLevel) -> Self {
    //         lock.0
    //     }
    // }

    // #[cfg(feature = "serde")]
    // impl<'de> serde::Deserialize<'de> for LockLevel {
    //     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    //     where
    //         D: serde::Deserializer<'de>,
    //     {
    //         let lock: LockLevelType = serde::Deserialize::deserialize(deserializer)?;
    //         lock.try_into().map_err(serde::de::Error::custom)
    //     }
    // }

    // impl Default for LockLevel {
    //     fn default() -> Self {
    //         LockLevel(MIN_LOCK_LEVEL)
    //     }
    // }

    /// According to ARINC 653P1-5 this may either be 32 or 64 bits.
    /// Internally we will use 64-bit by default.
    /// The implementing Hypervisor may cast this to 32-bit if needed
    pub type ProcessId = ApexLongInteger;
    pub const NULL_PROCESS_ID: ProcessId = 0;
    pub const MAIN_PROCESS_ID: ProcessId = -1;

    #[repr(u32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "strum", derive(strum::FromRepr))]
    pub enum ProcessState {
        Dormant = 0,
        Ready = 1,
        Running = 2,
        Waiting = 3,
        Faulted = 4,
    }

    #[repr(u32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "strum", derive(strum::FromRepr))]
    pub enum Deadline {
        Soft = 0,
        Hard = 1,
    }

    #[repr(C)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ApexProcessAttribute {
        pub period: ApexSystemTime,
        pub time_capacity: ApexSystemTime,
        pub entry_point: SystemAddress,
        pub stack_size: StackSize,
        pub base_priority: Priority,
        pub deadline: Deadline,
        pub name: ProcessName,
    }

    #[repr(C)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ApexProcessStatus {
        pub deadline_time: ApexSystemTime,
        pub current_priority: Priority,
        pub process_state: ProcessState,
        pub attributes: ApexProcessAttribute,
    }

    pub trait ApexProcessP4 {
        // Only during Warm/Cold-Start
        fn create_process<L: Locked>(
            attributes: &ApexProcessAttribute,
        ) -> Result<ProcessId, ErrorReturnCode>;

        fn start<L: Locked>(process_id: ProcessId) -> Result<(), ErrorReturnCode>;
    }

    pub trait ApexProcessP1: ApexProcessP4 {
        fn set_priority<L: Locked>(
            process_id: ProcessId,
            priority: Priority,
        ) -> Result<(), ErrorReturnCode>;

        fn suspend_self<L: Locked>(time_out: ApexSystemTime) -> Result<(), ErrorReturnCode>;

        fn suspend<L: Locked>(process_id: ProcessId) -> Result<(), ErrorReturnCode>;

        fn resume<L: Locked>(process_id: ProcessId) -> Result<(), ErrorReturnCode>;

        fn stop_self<L: Locked>();

        fn stop<L: Locked>(process_id: ProcessId) -> Result<(), ErrorReturnCode>;

        fn delayed_start<L: Locked>(
            process_id: ProcessId,
            delay_time: ApexSystemTime,
        ) -> Result<(), ErrorReturnCode>;

        fn lock_preemption<L: Locked>() -> Result<LockLevel, ErrorReturnCode>;

        fn unlock_preemption<L: Locked>() -> Result<LockLevel, ErrorReturnCode>;

        fn get_my_id<L: Locked>() -> Result<ProcessId, ErrorReturnCode>;

        fn get_process_id<L: Locked>(
            process_name: ProcessName,
        ) -> Result<ProcessId, ErrorReturnCode>;

        fn get_process_status<L: Locked>(
            process_id: ProcessId,
        ) -> Result<ApexProcessStatus, ErrorReturnCode>;

        // Only during Warm/Cold-Start
        fn initialize_process_core_affinity<L: Locked>(
            process_id: ProcessId,
            processor_core_id: ProcessorCoreId,
        ) -> Result<(), ErrorReturnCode>;

        fn get_my_processor_core_id<L: Locked>() -> ProcessorCoreId;

        fn get_my_index<L: Locked>() -> Result<ProcessIndex, ErrorReturnCode>;
    }
}

pub mod abstraction {
    use core::marker::PhantomData;

    // Reexport important basic-types for downstream-user
    pub use super::basic::{
        ApexProcessP1, ApexProcessP4, Deadline, LockLevel, Priority, ProcessId, ProcessIndex,
        ProcessName, StackSize,
    };
    use crate::bindings::*;
    use crate::hidden::Key;
    use crate::prelude::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ProcessAttribute {
        pub period: SystemTime,
        pub time_capacity: SystemTime,
        pub entry_point: SystemAddress,
        pub stack_size: StackSize,
        pub base_priority: Priority,
        pub deadline: Deadline,
        pub name: Name,
    }

    impl From<ProcessAttribute> for ApexProcessAttribute {
        fn from(p: ProcessAttribute) -> Self {
            ApexProcessAttribute {
                period: p.period.into(),
                time_capacity: p.time_capacity.into(),
                entry_point: p.entry_point,
                stack_size: p.stack_size,
                base_priority: p.base_priority,
                deadline: p.deadline,
                name: p.name.into(),
            }
        }
    }

    impl From<ApexProcessAttribute> for ProcessAttribute {
        fn from(p: ApexProcessAttribute) -> Self {
            ProcessAttribute {
                period: p.period.into(),
                time_capacity: p.time_capacity.into(),
                entry_point: p.entry_point,
                stack_size: p.stack_size,
                base_priority: p.base_priority,
                deadline: p.deadline,
                name: Name::new(p.name),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ProcessStatus {
        pub deadline_time: SystemTime,
        pub current_priority: Priority,
        pub process_state: super::basic::ProcessState,
        pub attributes: ProcessAttribute,
    }

    impl From<ApexProcessStatus> for ProcessStatus {
        fn from(p: ApexProcessStatus) -> Self {
            ProcessStatus {
                deadline_time: p.deadline_time.into(),
                current_priority: p.current_priority,
                process_state: p.process_state,
                attributes: p.attributes.into(),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Process<P: ApexProcessP4> {
        _p: PhantomData<P>,
        id: ProcessId,
    }

    pub trait ApexProcessP1Ext: ApexProcessP1 + Sized {
        fn get_process(name: Name) -> Result<Process<Self>, Error>;
    }

    impl<P: ApexProcessP1> ApexProcessP1Ext for P {
        fn get_process(name: Name) -> Result<Process<P>, Error> {
            let id = P::get_process_id::<Key>(name.into())?;
            Ok(Process {
                _p: Default::default(),
                id,
            })
        }
    }

    impl<P: ApexProcessP4> Process<P> {
        pub fn start(&self) -> Result<(), Error> {
            P::start::<Key>(self.id)?;
            Ok(())
        }

        pub fn id(&self) -> ProcessId {
            self.id
        }
    }

    impl<P: ApexProcessP1> Process<P> {
        pub fn from_name(name: Name) -> Result<Process<P>, Error> {
            P::get_process(name)
        }

        pub fn get_self() -> Result<Process<P>, Error> {
            let id = P::get_my_id::<Key>()?;
            Ok(Process {
                _p: Default::default(),
                id,
            })
        }

        pub fn set_priority(&self, priority: Priority) -> Result<(), Error> {
            P::set_priority::<Key>(self.id, priority)?;
            Ok(())
        }

        pub fn suspend_self(time_out: SystemTime) -> Result<(), Error> {
            P::suspend_self::<Key>(time_out.into())?;
            Ok(())
        }

        pub fn suspend(&self) -> Result<(), Error> {
            P::suspend::<Key>(self.id)?;
            Ok(())
        }

        pub fn resume(&self) -> Result<(), Error> {
            P::resume::<Key>(self.id)?;
            Ok(())
        }

        pub fn stop_self() {
            P::stop_self::<Key>()
        }

        pub fn stop(&self) -> Result<(), Error> {
            P::stop::<Key>(self.id)?;
            Ok(())
        }

        pub fn delayed_start(&self, delay_time: SystemTime) -> Result<(), Error> {
            P::delayed_start::<Key>(self.id, delay_time.into())?;
            Ok(())
        }

        pub fn lock_preemption() -> Result<LockLevel, Error> {
            Ok(P::lock_preemption::<Key>()?)
        }

        pub fn unlock_preemption() -> Result<LockLevel, Error> {
            Ok(P::unlock_preemption::<Key>()?)
        }

        pub fn status(&self) -> ProcessStatus {
            // According to ARINC653P1-5 3.3.2.2 this can only fail if the processId
            //  does not exist in the current partition.
            // But since we retrieve the processId directly from the hypervisor
            //  there is no possible way for it not existing
            P::get_process_status::<Key>(self.id).unwrap().into()
        }

        pub fn get_my_processor_core_id() -> ProcessorCoreId {
            P::get_my_processor_core_id::<Key>()
        }

        pub fn get_my_index() -> Result<ProcessIndex, Error> {
            Ok(P::get_my_index::<Key>()?)
        }
    }

    impl<P: ApexProcessP4> StartContext<P> {
        pub fn create_process(&mut self, attr: ProcessAttribute) -> Result<Process<P>, Error> {
            let id = P::create_process::<Key>(&attr.into())?;
            Ok(Process {
                _p: Default::default(),
                id,
            })
        }
    }

    impl<P: ApexProcessP1> StartContext<P> {
        pub fn initialize_process_core_affinity(
            &self,
            process: &Process<P>,
            processor_core_id: ProcessorCoreId,
        ) -> Result<(), Error> {
            P::initialize_process_core_affinity::<Key>(process.id, processor_core_id)?;
            Ok(())
        }
    }
}
