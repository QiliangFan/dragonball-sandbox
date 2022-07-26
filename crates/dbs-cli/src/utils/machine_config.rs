// Copyright (C) 2022 Alibaba Cloud. All rights reserved.
// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/// We only support this number of vpus for now. Mostly because we have set all vcpu related metrics as u8
pub const MAX_SUPPORTED_VCPUS: u8 = 254;

/// Memory hotplug value should have alignment in this size (unit: MiB)
pub const MEMORY_HOTPLUG_ALIGNMENT: u8 = 64;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum VmConfigError {
    /// cannot update the configuration of the booted micro-VM
    #[error("update operation is not allowed after boot")]
    UpdateNotAllowedPostBoot,

    /// The max vcpu count is invalid
    #[error("the vCPU number shouldn't be larger than {}", MAX_SUPPORTED_VCPUS)]
    VcpuCountExceedsMaximum,

    /// The vcpu count is invliad. (When hyper-threading enabled, the `cpu_count` must be either 1 or even.
    #[error("the vCPU number '{0}' can only be 1 or an even number when hyperthreading is enabled")]
    InvalidVcpuCount(u8),

    /// The threads_per_core is invalid. It should be either 1 or 2.
    #[error("the threads_per_core_number '{0}' can only be 1 or 2")]
    InvalidThreadsPerCore(u8),

    /// max vcpu count inferred from cpu topology(threads_per_core * cores_per_die * dies_per_socket * sockets) should be larger or equal to vcpu_count
    #[error("the max vcpu count inferred from cpu topology '{0}' (threads_per_core * cores_per_die * dies_per_socket * sockets) should be larger or equal to vcpu_count")]
    InvalidCpuTopology(u8),

    /// The max vcpu count is invalid.
    #[error(
    "the max vCPU number '{0}' shouldn't less than vCPU count and can only be 1 or an even number when hyperthreading is enabled"
    )]
    InvalidMaxVcpuCount(u8),

    /// The memory size is invalid. The memory can only be an unsigned integer.
    #[error("the memory size 0x{0:x}MiB is invalid")]
    InvalidMemorySize(usize),

    /// The hotplug memory size is invalid. The memory can only be an unsigned integer.
    #[error(
    "the hotplug memory size '{0}' (MiB) is invalid, must be multiple of {}",
    MEMORY_HOTPLUG_ALIGNMENT
    )]
    InvalidHotplugMemorySize(usize),

    /// The memory type is invalid.
    #[error("the memory type '{0}' is invalid")]
    InvalidMemType(String),

    /// The memory file path is invalid.
    #[error("the memory file path is invalid")]
    InvalidMemFilePath(String),

    /// NUMA region memory size is invalid
    #[error("Total size of memory in NUMA regions: {0}, should matches memory size in config")]
    InvalidNumaRegionMemorySize(usize),

    /// NUMA region vCPU count is invalid
    #[error("Total counts of vCPUs in NUMA regions: {0}, should matches max vcpu count in config")]
    InvalidNumaRegionCpuCount(u16),

    /// NUMA region vCPU count is invalid
    #[error("Max id of vCPUs in NUMA regions: {0}, should matches max vcpu count in config")]
    InvalidNumaRegionCpuMaxId(u16),
}