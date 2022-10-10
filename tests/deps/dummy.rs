use apex_rs::bindings::*;

pub struct Dummy;

impl ApexPartitionP4 for Dummy {
    fn get_partition_status<L: Locked>() -> ApexPartitionStatus {
        todo!()
    }

    fn set_partition_mode<L: Locked>(operating_mode: OperatingMode) -> Result<(), ErrorReturnCode> {
        todo!()
    }
}

impl ApexQueuingPortP1 for Dummy {
    fn get_queuing_port_id<L: Locked>(
        queuing_port_name: QueuingPortName,
    ) -> Result<QueuingPortId, ErrorReturnCode> {
        todo!()
    }
}

impl ApexQueuingPortP4 for Dummy {
    fn create_queuing_port<L: Locked>(
        queuing_port_name: QueuingPortName,
        max_message_size: MessageSize,
        max_nb_message: MessageRange,
        port_direction: PortDirection,
        queuing_discipline: QueuingDiscipline,
    ) -> Result<QueuingPortId, ErrorReturnCode> {
        todo!()
    }

    fn send_queuing_message<L: Locked>(
        queuing_port_id: QueuingPortId,
        message: &[ApexByte],
        time_out: ApexSystemTime,
    ) -> Result<(), ErrorReturnCode> {
        todo!()
    }

    unsafe fn receive_queuing_message<L: Locked>(
        queuing_port_id: QueuingPortId,
        time_out: ApexSystemTime,
        message: &mut [ApexByte],
    ) -> Result<MessageSize, ErrorReturnCode> {
        todo!()
    }

    fn get_queuing_port_status<L: Locked>(
        queuing_port_id: QueuingPortId,
    ) -> Result<QueuingPortStatus, ErrorReturnCode> {
        todo!()
    }

    fn clear_queuing_port<L: Locked>(
        queuing_port_id: QueuingPortId,
    ) -> Result<(), ErrorReturnCode> {
        todo!()
    }
}

impl ApexSamplingPortP4 for Dummy {
    fn create_sampling_port<L: Locked>(
        sampling_port_name: SamplingPortName,
        max_message_size: MessageSize,
        port_direction: PortDirection,
        refresh_period: ApexSystemTime,
    ) -> Result<SamplingPortId, ErrorReturnCode> {
        todo!()
    }

    fn write_sampling_message<L: Locked>(
        sampling_port_id: SamplingPortId,
        message: &[ApexByte],
    ) -> Result<(), ErrorReturnCode> {
        todo!()
    }

    unsafe fn read_sampling_message<L: Locked>(
        sampling_port_id: SamplingPortId,
        message: &mut [ApexByte],
    ) -> Result<(Validity, MessageSize), ErrorReturnCode> {
        todo!()
    }
}

impl ApexSamplingPortP1 for Dummy {
    fn get_sampling_port_id<L: Locked>(
        sampling_port_name: SamplingPortName,
    ) -> Result<SamplingPortId, ErrorReturnCode> {
        todo!()
    }

    fn get_sampling_port_status<L: Locked>(
        sampling_port_id: SamplingPortId,
    ) -> Result<ApexSamplingPortStatus, ErrorReturnCode> {
        todo!()
    }
}

impl ApexProcessP4 for Dummy {
    fn create_process<L: Locked>(
        attributes: &ApexProcessAttribute,
    ) -> Result<ProcessId, ErrorReturnCode> {
        todo!()
    }

    fn start<L: Locked>(process_id: ProcessId) -> Result<(), ErrorReturnCode> {
        todo!()
    }
}

impl ApexProcessP1 for Dummy {
    fn set_priority<L: Locked>(
        process_id: ProcessId,
        priority: Priority,
    ) -> Result<(), ErrorReturnCode> {
        todo!()
    }

    fn suspend_self<L: Locked>(time_out: ApexSystemTime) -> Result<(), ErrorReturnCode> {
        todo!()
    }

    fn suspend<L: Locked>(process_id: ProcessId) -> Result<(), ErrorReturnCode> {
        todo!()
    }

    fn resume<L: Locked>(process_id: ProcessId) -> Result<(), ErrorReturnCode> {
        todo!()
    }

    fn stop_self<L: Locked>() {
        todo!()
    }

    fn stop<L: Locked>(process_id: ProcessId) -> Result<(), ErrorReturnCode> {
        todo!()
    }

    fn delayed_start<L: Locked>(
        process_id: ProcessId,
        delay_time: ApexSystemTime,
    ) -> Result<(), ErrorReturnCode> {
        todo!()
    }

    fn lock_preemption<L: Locked>() -> Result<LockLevel, ErrorReturnCode> {
        todo!()
    }

    fn unlock_preemption<L: Locked>() -> Result<LockLevel, ErrorReturnCode> {
        todo!()
    }

    fn get_my_id<L: Locked>() -> Result<ProcessId, ErrorReturnCode> {
        todo!()
    }

    fn get_process_id<L: Locked>(process_name: ProcessName) -> Result<ProcessId, ErrorReturnCode> {
        todo!()
    }

    fn get_process_status<L: Locked>(
        process_id: ProcessId,
    ) -> Result<ApexProcessStatus, ErrorReturnCode> {
        todo!()
    }

    fn initialize_process_core_affinity<L: Locked>(
        process_id: ProcessId,
        processor_core_id: ProcessorCoreId,
    ) -> Result<(), ErrorReturnCode> {
        todo!()
    }

    fn get_my_processor_core_id<L: Locked>() -> ProcessorCoreId {
        todo!()
    }

    fn get_my_index<L: Locked>() -> Result<ProcessIndex, ErrorReturnCode> {
        todo!()
    }
}
