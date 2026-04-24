use crate::metadata::*;
pub const METADATA: Metadata = Metadata {
    name: "MCXA156",
    pins: PINS,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
};
pub const PINS: &[Pin] = &[
    Pin {
        name: "P1_8",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_9",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_10",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_11",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_12",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_13",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_14",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_15",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_29",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_30",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_31",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P4_2",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P4_3",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P4_4",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P4_5",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P4_6",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P4_7",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_0",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_1",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_2",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_3",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_4",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_5",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_6",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_7",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_10",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_11",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_12",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_13",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_15",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_16",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_17",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_19",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_20",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_21",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_23",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_31",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_30",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_29",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_28",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_27",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_22",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_21",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_20",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_19",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_18",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_17",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_16",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_15",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_14",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_13",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_12",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_11",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_10",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_9",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_8",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_7",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_6",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_2",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_1",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_0",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_0",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_1",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_2",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_3",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_6",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_16",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_17",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_18",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_19",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_20",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_21",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_22",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_23",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_0",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_1",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_2",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_3",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_4",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_5",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_6",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_7",
        iomuxc: None,
        feature: None,
    },
];
pub const PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "INPUTMUX0",
        address: 0x40001000,
        driver_name: "mcxa/INPUTMUX",
        signals: &[
            Signal {
                name: "out2",
                pins: &[
                    SignalPin {
                        pin: "P1_11",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_7",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "in3",
                pins: &[
                    SignalPin {
                        pin: "P1_13",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_8",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "out3",
                pins: &[
                    SignalPin {
                        pin: "P1_30",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_5",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "in4",
                pins: &[
                    SignalPin {
                        pin: "P1_31",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_6",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_11",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_9",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "in5",
                pins: &[
                    SignalPin {
                        pin: "P4_7",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_7",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_10",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "in6",
                pins: &[
                    SignalPin {
                        pin: "P2_0",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_2",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_11",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "in7",
                pins: &[
                    SignalPin {
                        pin: "P2_1",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_3",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "out4",
                pins: &[
                    SignalPin {
                        pin: "P2_6",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_15",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "out5",
                pins: &[
                    SignalPin {
                        pin: "P2_10",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_19",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_23",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "in8",
                pins: &[
                    SignalPin {
                        pin: "P2_13",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_20",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "in9",
                pins: &[
                    SignalPin {
                        pin: "P2_17",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_21",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "in10",
                pins: &[SignalPin {
                    pin: "P3_31",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "out6",
                pins: &[SignalPin {
                    pin: "P3_30",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "in11",
                pins: &[SignalPin {
                    pin: "P3_28",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "out7",
                pins: &[SignalPin {
                    pin: "P3_27",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "out1",
                pins: &[
                    SignalPin {
                        pin: "P3_21",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_3",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "out0",
                pins: &[
                    SignalPin {
                        pin: "P3_20",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_2",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "in2",
                pins: &[
                    SignalPin {
                        pin: "P3_7",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_6",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "in1",
                pins: &[
                    SignalPin {
                        pin: "P3_1",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_1",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "in0",
                pins: &[
                    SignalPin {
                        pin: "P3_0",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_0",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "I3C0",
        address: 0x40002000,
        driver_name: "mcxa/I3C",
        signals: &[
            Signal {
                name: "sda",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_30",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_16",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "scl",
                pins: &[
                    SignalPin {
                        pin: "P1_9",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_31",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_17",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "pur",
                pins: &[
                    SignalPin {
                        pin: "P1_11",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_2",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "I3C0Rx",
                mux: "DMA0",
                request: 7,
            },
            DmaMux {
                signal: "I3C0Tx",
                mux: "DMA0",
                request: 8,
            },
        ],
    },
    Peripheral {
        name: "CTIMER0",
        address: 0x40004000,
        driver_name: "mcxa/CTIMER",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "CTIMER0M0",
                mux: "DMA0",
                request: 31,
            },
            DmaMux {
                signal: "CTIMER0M1",
                mux: "DMA0",
                request: 32,
            },
        ],
    },
    Peripheral {
        name: "CTIMER1",
        address: 0x40005000,
        driver_name: "mcxa/CTIMER",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "CTIMER1M0",
                mux: "DMA1",
                request: 33,
            },
            DmaMux {
                signal: "CTIMER1M1",
                mux: "DMA1",
                request: 34,
            },
        ],
    },
    Peripheral {
        name: "CTIMER2",
        address: 0x40006000,
        driver_name: "mcxa/CTIMER",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "CTIMER2M0",
                mux: "DMA2",
                request: 35,
            },
            DmaMux {
                signal: "CTIMER2M1",
                mux: "DMA2",
                request: 36,
            },
        ],
    },
    Peripheral {
        name: "CTIMER3",
        address: 0x40007000,
        driver_name: "mcxa/CTIMER",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "CTIMER3M0",
                mux: "DMA3",
                request: 37,
            },
            DmaMux {
                signal: "CTIMER3M1",
                mux: "DMA3",
                request: 38,
            },
        ],
    },
    Peripheral {
        name: "CTIMER4",
        address: 0x40008000,
        driver_name: "mcxa/CTIMER",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "CTIMER4M0",
                mux: "DMA4",
                request: 39,
            },
            DmaMux {
                signal: "CTIMER4M1",
                mux: "DMA4",
                request: 40,
            },
        ],
    },
    Peripheral {
        name: "FREQME0",
        address: 0x40009000,
        driver_name: "mcxa/FREQME",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "UTICK0",
        address: 0x4000B000,
        driver_name: "mcxa/UTICK",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "WWDT0",
        address: 0x4000C000,
        driver_name: "mcxa/WWDT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "DMA0",
        address: 0x40080000,
        driver_name: "mcxa/DMA::DMA8",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "EDMA_0_TCD0",
        address: 0x40081000,
        driver_name: "mcxa/EDMA_TCD::TCD8",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "AOI0",
        address: 0x40089000,
        driver_name: "mcxa/AOI",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "AOI1",
        address: 0x40097000,
        driver_name: "mcxa/AOI",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "CRC0",
        address: 0x4008A000,
        driver_name: "mcxa/CRC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "CMC",
        address: 0x4008B000,
        driver_name: "mcxa/CMC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "EIM0",
        address: 0x4008C000,
        driver_name: "mcxa/EIM",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "ERM0",
        address: 0x4008D000,
        driver_name: "mcxa/ERM",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "MBC0",
        address: 0x4008E000,
        driver_name: "mcxa/MBC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "SCG0",
        address: 0x4008F000,
        driver_name: "mcxa/SCG",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "SPC0",
        address: 0x40090000,
        driver_name: "mcxa/SPC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "MRCC0",
        address: 0x40091000,
        driver_name: "mcxa/MRCC1xx::MRCC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "SYSCON",
        address: 0x40091000,
        driver_name: "mcxa/SYSCON1xx::SYSCON",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "GLIKEY0",
        address: 0x40091D00,
        driver_name: "mcxa/GLIKEY",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "WUU0",
        address: 0x40092000,
        driver_name: "mcxa/WUU",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "WUU0WakeUpEvent",
            mux: "DMA0",
            request: 1,
        }],
    },
    Peripheral {
        name: "VBAT0",
        address: 0x40093000,
        driver_name: "mcxa/VBAT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "FMC0",
        address: 0x40094000,
        driver_name: "mcxa/FMC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "FMU0",
        address: 0x40095000,
        driver_name: "mcxa/FMU",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "FLEXIO0",
        address: 0x40099000,
        driver_name: "mcxa/FLEXIO",
        signals: &[
            Signal {
                name: "d16",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_8",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d17",
                pins: &[
                    SignalPin {
                        pin: "P1_9",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_9",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d18",
                pins: &[
                    SignalPin {
                        pin: "P1_10",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_10",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_10",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d19",
                pins: &[
                    SignalPin {
                        pin: "P1_11",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_11",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_11",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d20",
                pins: &[
                    SignalPin {
                        pin: "P1_12",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_12",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_12",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d21",
                pins: &[
                    SignalPin {
                        pin: "P1_13",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_13",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_13",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d22",
                pins: &[
                    SignalPin {
                        pin: "P1_14",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_14",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d23",
                pins: &[
                    SignalPin {
                        pin: "P1_15",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_15",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_15",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d30",
                pins: &[
                    SignalPin {
                        pin: "P1_30",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_30",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_22",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d31",
                pins: &[
                    SignalPin {
                        pin: "P1_31",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_23",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_31",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d10",
                pins: &[
                    SignalPin {
                        pin: "P4_2",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_2",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_2",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_2",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d11",
                pins: &[
                    SignalPin {
                        pin: "P4_3",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_3",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_3",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d12",
                pins: &[
                    SignalPin {
                        pin: "P4_4",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_4",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_4",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d13",
                pins: &[
                    SignalPin {
                        pin: "P4_5",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_5",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_5",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d14",
                pins: &[
                    SignalPin {
                        pin: "P4_6",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_6",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_6",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_6",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d15",
                pins: &[
                    SignalPin {
                        pin: "P4_7",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_7",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_7",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_7",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d8",
                pins: &[
                    SignalPin {
                        pin: "P2_0",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_0",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_0",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d9",
                pins: &[
                    SignalPin {
                        pin: "P2_1",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_1",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_1",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d24",
                pins: &[
                    SignalPin {
                        pin: "P2_16",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_16",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d25",
                pins: &[
                    SignalPin {
                        pin: "P2_17",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_17",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d27",
                pins: &[
                    SignalPin {
                        pin: "P2_19",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_27",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_19",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d28",
                pins: &[
                    SignalPin {
                        pin: "P2_20",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_28",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_20",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d29",
                pins: &[
                    SignalPin {
                        pin: "P2_21",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_29",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_21",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d26",
                pins: &[SignalPin {
                    pin: "P3_18",
                    alt: 6u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d0",
                pins: &[
                    SignalPin {
                        pin: "P0_0",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_16",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d1",
                pins: &[
                    SignalPin {
                        pin: "P0_1",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_17",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d2",
                pins: &[
                    SignalPin {
                        pin: "P0_2",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_18",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d3",
                pins: &[
                    SignalPin {
                        pin: "P0_3",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_19",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d6",
                pins: &[
                    SignalPin {
                        pin: "P0_6",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_22",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d4",
                pins: &[SignalPin {
                    pin: "P0_20",
                    alt: 6u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d5",
                pins: &[SignalPin {
                    pin: "P0_21",
                    alt: 6u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "d7",
                pins: &[SignalPin {
                    pin: "P0_23",
                    alt: 6u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "FLEXIO0SR0",
                mux: "DMA0",
                request: 71,
            },
            DmaMux {
                signal: "FLEXIO0SR1",
                mux: "DMA0",
                request: 72,
            },
            DmaMux {
                signal: "FLEXIO0SR2",
                mux: "DMA0",
                request: 73,
            },
            DmaMux {
                signal: "FLEXIO0SR3",
                mux: "DMA0",
                request: 74,
            },
        ],
    },
    Peripheral {
        name: "LPI2C0",
        address: 0x4009A000,
        driver_name: "mcxa/LPI2C",
        signals: &[
            Signal {
                name: "sda",
                pins: &[
                    SignalPin {
                        pin: "P1_30",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_16",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "scl",
                pins: &[
                    SignalPin {
                        pin: "P1_31",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_17",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "hreq",
                pins: &[SignalPin {
                    pin: "P0_6",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "scls",
                pins: &[SignalPin {
                    pin: "P0_18",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "sdas",
                pins: &[SignalPin {
                    pin: "P0_19",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPI2C0Rx",
                mux: "DMA0",
                request: 11,
            },
            DmaMux {
                signal: "LPI2C0Tx",
                mux: "DMA0",
                request: 12,
            },
        ],
    },
    Peripheral {
        name: "LPI2C1",
        address: 0x4009B000,
        driver_name: "mcxa/LPI2C",
        signals: &[
            Signal {
                name: "sda",
                pins: &[
                    SignalPin {
                        pin: "P1_12",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_0",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "scl",
                pins: &[
                    SignalPin {
                        pin: "P1_13",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_1",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "scls",
                pins: &[
                    SignalPin {
                        pin: "P1_14",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_3",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "sdas",
                pins: &[
                    SignalPin {
                        pin: "P1_15",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_2",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPI2C1Rx",
                mux: "DMA1",
                request: 13,
            },
            DmaMux {
                signal: "LPI2C1Tx",
                mux: "DMA1",
                request: 14,
            },
        ],
    },
    Peripheral {
        name: "LPI2C2",
        address: 0x400D4000,
        driver_name: "mcxa/LPI2C",
        signals: &[
            Signal {
                name: "sda",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_4",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "scl",
                pins: &[
                    SignalPin {
                        pin: "P1_9",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_3",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "sdas",
                pins: &[
                    SignalPin {
                        pin: "P1_10",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_2",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "scls",
                pins: &[
                    SignalPin {
                        pin: "P1_11",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_5",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "hreq",
                pins: &[SignalPin {
                    pin: "P4_6",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPI2C2Rx",
                mux: "DMA2",
                request: 3,
            },
            DmaMux {
                signal: "LPI2C2Tx",
                mux: "DMA2",
                request: 4,
            },
        ],
    },
    Peripheral {
        name: "LPI2C3",
        address: 0x400D5000,
        driver_name: "mcxa/LPI2C",
        signals: &[
            Signal {
                name: "sdas",
                pins: &[SignalPin {
                    pin: "P3_31",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "scls",
                pins: &[SignalPin {
                    pin: "P3_30",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "hreq",
                pins: &[SignalPin {
                    pin: "P3_29",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "sda",
                pins: &[
                    SignalPin {
                        pin: "P3_28",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_20",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "scl",
                pins: &[
                    SignalPin {
                        pin: "P3_27",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_21",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPI2C3Rx",
                mux: "DMA3",
                request: 5,
            },
            DmaMux {
                signal: "LPI2C3Tx",
                mux: "DMA3",
                request: 6,
            },
        ],
    },
    Peripheral {
        name: "LPSPI0",
        address: 0x4009C000,
        driver_name: "mcxa/LPSPI",
        signals: &[
            Signal {
                name: "pcs0",
                pins: &[
                    SignalPin {
                        pin: "P0_0",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_3",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "sdi",
                pins: &[
                    SignalPin {
                        pin: "P0_1",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_2",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "sck",
                pins: &[
                    SignalPin {
                        pin: "P0_2",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_1",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "sdo",
                pins: &[
                    SignalPin {
                        pin: "P0_3",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_0",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "pcs1",
                pins: &[
                    SignalPin {
                        pin: "P0_6",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_6",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "pcs2",
                pins: &[
                    SignalPin {
                        pin: "P0_16",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_5",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "pcs3",
                pins: &[
                    SignalPin {
                        pin: "P0_17",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_4",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPSPI0Rx",
                mux: "DMA0",
                request: 15,
            },
            DmaMux {
                signal: "LPSPI0Tx",
                mux: "DMA0",
                request: 16,
            },
        ],
    },
    Peripheral {
        name: "LPSPI1",
        address: 0x4009D000,
        driver_name: "mcxa/LPSPI",
        signals: &[
            Signal {
                name: "pcs1",
                pins: &[
                    SignalPin {
                        pin: "P2_6",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_2",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "sck",
                pins: &[
                    SignalPin {
                        pin: "P2_12",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_10",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "sdo",
                pins: &[
                    SignalPin {
                        pin: "P2_13",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_8",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "sdi",
                pins: &[
                    SignalPin {
                        pin: "P2_15",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_16",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_9",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "pcs0",
                pins: &[
                    SignalPin {
                        pin: "P2_17",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_11",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "pcs2",
                pins: &[
                    SignalPin {
                        pin: "P2_20",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_7",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "pcs3",
                pins: &[
                    SignalPin {
                        pin: "P2_21",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_6",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPSPI1Rx",
                mux: "DMA1",
                request: 17,
            },
            DmaMux {
                signal: "LPSPI1Tx",
                mux: "DMA1",
                request: 18,
            },
        ],
    },
    Peripheral {
        name: "LPUART0",
        address: 0x4009F000,
        driver_name: "mcxa/LPUART",
        signals: &[
            Signal {
                name: "rxd",
                pins: &[
                    SignalPin {
                        pin: "P2_0",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_2",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_20",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "txd",
                pins: &[
                    SignalPin {
                        pin: "P2_1",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_3",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_21",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "b",
                pins: &[
                    SignalPin {
                        pin: "P2_2",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_3",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_0",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_1",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_22",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_23",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPUART0Rx",
                mux: "DMA0",
                request: 21,
            },
            DmaMux {
                signal: "LPUART0Tx",
                mux: "DMA0",
                request: 22,
            },
        ],
    },
    Peripheral {
        name: "LPUART1",
        address: 0x400A0000,
        driver_name: "mcxa/LPUART",
        signals: &[
            Signal {
                name: "rxd",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_12",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_20",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_8",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "txd",
                pins: &[
                    SignalPin {
                        pin: "P1_9",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_13",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_21",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_9",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "b",
                pins: &[
                    SignalPin {
                        pin: "P1_10",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_11",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_15",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_16",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_17",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_22",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_11",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_10",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPUART1Rx",
                mux: "DMA1",
                request: 23,
            },
            DmaMux {
                signal: "LPUART1Tx",
                mux: "DMA1",
                request: 24,
            },
        ],
    },
    Peripheral {
        name: "LPUART2",
        address: 0x400A1000,
        driver_name: "mcxa/LPUART",
        signals: &[
            Signal {
                name: "rxd",
                pins: &[
                    SignalPin {
                        pin: "P1_12",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_3",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_11",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_14",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_4",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "txd",
                pins: &[
                    SignalPin {
                        pin: "P1_13",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_2",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_10",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_15",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_5",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "b",
                pins: &[
                    SignalPin {
                        pin: "P1_14",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_15",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_4",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_5",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_13",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_12",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_6",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_7",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPUART2Rx",
                mux: "DMA2",
                request: 25,
            },
            DmaMux {
                signal: "LPUART2Tx",
                mux: "DMA2",
                request: 26,
            },
        ],
    },
    Peripheral {
        name: "LPUART3",
        address: 0x400A2000,
        driver_name: "mcxa/LPUART",
        signals: &[
            Signal {
                name: "rxd",
                pins: &[
                    SignalPin {
                        pin: "P4_2",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_13",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_0",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "txd",
                pins: &[
                    SignalPin {
                        pin: "P4_5",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_12",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_1",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "b",
                pins: &[
                    SignalPin {
                        pin: "P4_6",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_7",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_15",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_14",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_7",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_6",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPUART3Rx",
                mux: "DMA3",
                request: 27,
            },
            DmaMux {
                signal: "LPUART3Tx",
                mux: "DMA3",
                request: 28,
            },
        ],
    },
    Peripheral {
        name: "LPUART4",
        address: 0x400A3000,
        driver_name: "mcxa/LPUART",
        signals: &[
            Signal {
                name: "txd",
                pins: &[
                    SignalPin {
                        pin: "P4_3",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_7",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_27",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_19",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "rxd",
                pins: &[
                    SignalPin {
                        pin: "P4_4",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_6",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_28",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_18",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "b",
                pins: &[
                    SignalPin {
                        pin: "P2_0",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_1",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_31",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_30",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_17",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_16",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPUART4Rx",
                mux: "DMA4",
                request: 29,
            },
            DmaMux {
                signal: "LPUART4Tx",
                mux: "DMA4",
                request: 30,
            },
        ],
    },
    Peripheral {
        name: "USB0",
        address: 0x400A4000,
        driver_name: "mcxa/USB",
        signals: &[Signal {
            name: "det",
            pins: &[SignalPin {
                pin: "P2_12",
                alt: 1u8,
                iomuxc_daisy: None,
            }],
            iomuxc_daisy: None,
        }],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "QDC0",
        address: 0x400A7000,
        driver_name: "mcxa/QDC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "QDC0",
            mux: "DMA0",
            request: 65,
        }],
    },
    Peripheral {
        name: "QDC1",
        address: 0x400A8000,
        driver_name: "mcxa/QDC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "QDC1",
            mux: "DMA1",
            request: 66,
        }],
    },
    Peripheral {
        name: "FLEXPWM0",
        address: 0x400A9000,
        driver_name: "mcxa/FLEXPWM",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "FLEXPWM0capt0",
                mux: "DMA0",
                request: 41,
            },
            DmaMux {
                signal: "FLEXPWM0capt1",
                mux: "DMA0",
                request: 42,
            },
            DmaMux {
                signal: "FLEXPWM0capt2",
                mux: "DMA0",
                request: 43,
            },
            DmaMux {
                signal: "FLEXPWM0val0",
                mux: "DMA0",
                request: 45,
            },
            DmaMux {
                signal: "FLEXPWM0val1",
                mux: "DMA0",
                request: 46,
            },
            DmaMux {
                signal: "FLEXPWM0val2",
                mux: "DMA0",
                request: 47,
            },
        ],
    },
    Peripheral {
        name: "FLEXPWM1",
        address: 0x400AA000,
        driver_name: "mcxa/FLEXPWM",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "FLEXPWM1capt0",
                mux: "DMA1",
                request: 79,
            },
            DmaMux {
                signal: "FLEXPWM1capt1",
                mux: "DMA1",
                request: 80,
            },
            DmaMux {
                signal: "FLEXPWM1capt2",
                mux: "DMA1",
                request: 81,
            },
            DmaMux {
                signal: "FLEXPWM1val0",
                mux: "DMA1",
                request: 83,
            },
            DmaMux {
                signal: "FLEXPWM1val1",
                mux: "DMA1",
                request: 84,
            },
            DmaMux {
                signal: "FLEXPWM1val2",
                mux: "DMA1",
                request: 85,
            },
        ],
    },
    Peripheral {
        name: "LPTMR0",
        address: 0x400AB000,
        driver_name: "mcxa/LPTMR",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "LPTMR0CounterMatchEvent",
            mux: "DMA0",
            request: 49,
        }],
    },
    Peripheral {
        name: "OSTIMER0",
        address: 0x400AD000,
        driver_name: "mcxa/OSTIMER",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "WAKETIMER0",
        address: 0x400AE000,
        driver_name: "mcxa/WAKETIMER",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "ADC0",
        address: 0x400AF000,
        driver_name: "mcxa/ADC",
        signals: &[
            Signal {
                name: "a0",
                pins: &[SignalPin {
                    pin: "P2_0",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a1",
                pins: &[SignalPin {
                    pin: "P2_1",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a4",
                pins: &[SignalPin {
                    pin: "P2_2",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a7",
                pins: &[SignalPin {
                    pin: "P2_7",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a5",
                pins: &[SignalPin {
                    pin: "P2_12",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a2",
                pins: &[SignalPin {
                    pin: "P2_15",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a6",
                pins: &[SignalPin {
                    pin: "P2_16",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a14",
                pins: &[SignalPin {
                    pin: "P0_3",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a15",
                pins: &[SignalPin {
                    pin: "P0_6",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a8",
                pins: &[SignalPin {
                    pin: "P0_18",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a9",
                pins: &[SignalPin {
                    pin: "P0_19",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a10",
                pins: &[SignalPin {
                    pin: "P0_20",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a11",
                pins: &[SignalPin {
                    pin: "P0_21",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a12",
                pins: &[SignalPin {
                    pin: "P0_22",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a13",
                pins: &[SignalPin {
                    pin: "P0_23",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a16",
                pins: &[SignalPin {
                    pin: "P1_0",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a17",
                pins: &[SignalPin {
                    pin: "P1_1",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a18",
                pins: &[SignalPin {
                    pin: "P1_2",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a19",
                pins: &[SignalPin {
                    pin: "P1_3",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a20",
                pins: &[SignalPin {
                    pin: "P1_4",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a21",
                pins: &[SignalPin {
                    pin: "P1_5",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a22",
                pins: &[SignalPin {
                    pin: "P1_6",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a23",
                pins: &[SignalPin {
                    pin: "P1_7",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "ADC0FifoRequest",
            mux: "DMA0",
            request: 51,
        }],
    },
    Peripheral {
        name: "ADC1",
        address: 0x400B0000,
        driver_name: "mcxa/ADC",
        signals: &[
            Signal {
                name: "a8",
                pins: &[SignalPin {
                    pin: "P1_10",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a9",
                pins: &[SignalPin {
                    pin: "P1_11",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a10",
                pins: &[SignalPin {
                    pin: "P1_12",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a11",
                pins: &[SignalPin {
                    pin: "P1_13",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a12",
                pins: &[SignalPin {
                    pin: "P1_14",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a13",
                pins: &[SignalPin {
                    pin: "P1_15",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a4",
                pins: &[SignalPin {
                    pin: "P2_3",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a0",
                pins: &[SignalPin {
                    pin: "P2_4",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a1",
                pins: &[SignalPin {
                    pin: "P2_5",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a3",
                pins: &[SignalPin {
                    pin: "P2_6",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a7",
                pins: &[SignalPin {
                    pin: "P2_7",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a5",
                pins: &[SignalPin {
                    pin: "P2_13",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a6",
                pins: &[SignalPin {
                    pin: "P2_17",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a2",
                pins: &[SignalPin {
                    pin: "P2_19",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a20",
                pins: &[SignalPin {
                    pin: "P3_31",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a21",
                pins: &[SignalPin {
                    pin: "P3_30",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "a22",
                pins: &[SignalPin {
                    pin: "P3_29",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "ADC1FifoRequest",
            mux: "DMA1",
            request: 52,
        }],
    },
    Peripheral {
        name: "CMP0",
        address: 0x400B1000,
        driver_name: "mcxa/CMP",
        signals: &[
            Signal {
                name: "in0",
                pins: &[SignalPin {
                    pin: "P2_2",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "out",
                pins: &[
                    SignalPin {
                        pin: "P0_3",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_18",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "in3",
                pins: &[SignalPin {
                    pin: "P1_0",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "in1",
                pins: &[SignalPin {
                    pin: "P1_3",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "in2",
                pins: &[SignalPin {
                    pin: "P1_4",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "CMP0",
            mux: "DMA0",
            request: 53,
        }],
    },
    Peripheral {
        name: "CMP1",
        address: 0x400B2000,
        driver_name: "mcxa/CMP",
        signals: &[
            Signal {
                name: "in0",
                pins: &[SignalPin {
                    pin: "P2_3",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "in1",
                pins: &[SignalPin {
                    pin: "P0_3",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "out",
                pins: &[
                    SignalPin {
                        pin: "P0_6",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_19",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "in3",
                pins: &[SignalPin {
                    pin: "P1_1",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "in2",
                pins: &[SignalPin {
                    pin: "P1_5",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "CMP1",
            mux: "DMA1",
            request: 54,
        }],
    },
    Peripheral {
        name: "DAC0",
        address: 0x400B4000,
        driver_name: "mcxa/DAC",
        signals: &[Signal {
            name: "out",
            pins: &[SignalPin {
                pin: "P2_2",
                alt: 0u8,
                iomuxc_daisy: None,
            }],
            iomuxc_daisy: None,
        }],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "DAC0FifoRequest",
            mux: "DMA0",
            request: 56,
        }],
    },
    Peripheral {
        name: "OPAMP0",
        address: 0x400B7000,
        driver_name: "mcxa/OPAMP",
        signals: &[
            Signal {
                name: "inp0",
                pins: &[SignalPin {
                    pin: "P2_12",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "inn",
                pins: &[SignalPin {
                    pin: "P2_13",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "out",
                pins: &[SignalPin {
                    pin: "P2_15",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "PORT0",
        address: 0x400BC000,
        driver_name: "mcxa/PORT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "PORT1",
        address: 0x400BD000,
        driver_name: "mcxa/PORT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "PORT2",
        address: 0x400BE000,
        driver_name: "mcxa/PORT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "PORT3",
        address: 0x400BF000,
        driver_name: "mcxa/PORT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "PORT4",
        address: 0x400C0000,
        driver_name: "mcxa/PORT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "CAN0",
        address: 0x400CC000,
        driver_name: "mcxa/CAN",
        signals: &[
            Signal {
                name: "txd",
                pins: &[
                    SignalPin {
                        pin: "P1_10",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_13",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_13",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_2",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_6",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "rxd",
                pins: &[
                    SignalPin {
                        pin: "P1_11",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_12",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_12",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_3",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_7",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "CAN0",
            mux: "DMA0",
            request: 2,
        }],
    },
    Peripheral {
        name: "CDOG",
        address: 0x40100000,
        driver_name: "mcxa/CDOG",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "DBGMAILBOX",
        address: 0x40101000,
        driver_name: "mcxa/DBGMAILBOX",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "GPIO0",
        address: 0x40102000,
        driver_name: "mcxa/GPIO",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "GPIO0PinEvent0",
            mux: "DMA0",
            request: 60,
        }],
    },
    Peripheral {
        name: "GPIO1",
        address: 0x40103000,
        driver_name: "mcxa/GPIO",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "GPIO1PinEvent0",
            mux: "DMA1",
            request: 61,
        }],
    },
    Peripheral {
        name: "GPIO2",
        address: 0x40104000,
        driver_name: "mcxa/GPIO",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "GPIO2PinEvent0",
            mux: "DMA2",
            request: 62,
        }],
    },
    Peripheral {
        name: "GPIO3",
        address: 0x40105000,
        driver_name: "mcxa/GPIO",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "GPIO3PinEvent0",
            mux: "DMA3",
            request: 63,
        }],
    },
    Peripheral {
        name: "GPIO4",
        address: 0x40106000,
        driver_name: "mcxa/GPIO",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "GPIO4PinEvent0",
            mux: "DMA4",
            request: 64,
        }],
    },
    Peripheral {
        name: "SCnSCB",
        address: 0xE000E000,
        driver_name: "mcxa/SCnSCB",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "SysTick",
        address: 0xE000E010,
        driver_name: "mcxa/SysTick",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "NVIC",
        address: 0xE000E100,
        driver_name: "mcxa/NVIC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "SCB",
        address: 0xE000ED00,
        driver_name: "mcxa/SCB",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "SAU",
        address: 0xE000EDD0,
        driver_name: "mcxa/SAU",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
];
pub const INTERRUPTS: &[(&str, u32)] = &[
    ("RESERVED16", 0u32),
    ("CMC", 1u32),
    ("DMA_CH0", 2u32),
    ("DMA_CH1", 3u32),
    ("DMA_CH2", 4u32),
    ("DMA_CH3", 5u32),
    ("DMA_CH4", 6u32),
    ("DMA_CH5", 7u32),
    ("DMA_CH6", 8u32),
    ("DMA_CH7", 9u32),
    ("ERM0_SINGLE_BIT", 10u32),
    ("ERM0_MULTI_BIT", 11u32),
    ("FMU0", 12u32),
    ("GLIKEY0", 13u32),
    ("MBC0", 14u32),
    ("SCG0", 15u32),
    ("SPC0", 16u32),
    ("VBAT0", 17u32),
    ("WUU0", 18u32),
    ("CAN0", 19u32),
    ("RESERVED36", 20u32),
    ("RESERVED37", 21u32),
    ("RESERVED38", 22u32),
    ("FLEXIO", 23u32),
    ("I3C0", 24u32),
    ("RESERVED41", 25u32),
    ("LPI2C0", 26u32),
    ("LPI2C1", 27u32),
    ("LPSPI0", 28u32),
    ("LPSPI1", 29u32),
    ("RESERVED46", 30u32),
    ("LPUART0", 31u32),
    ("LPUART1", 32u32),
    ("LPUART2", 33u32),
    ("LPUART3", 34u32),
    ("LPUART4", 35u32),
    ("USB0", 36u32),
    ("RESERVED53", 37u32),
    ("CDOG0", 38u32),
    ("CTIMER0", 39u32),
    ("CTIMER1", 40u32),
    ("CTIMER2", 41u32),
    ("CTIMER3", 42u32),
    ("CTIMER4", 43u32),
    ("FLEXPWM0_RELOAD_ERROR", 44u32),
    ("FLEXPWM0_FAULT", 45u32),
    ("FLEXPWM0_SUBMODULE0", 46u32),
    ("FLEXPWM0_SUBMODULE1", 47u32),
    ("FLEXPWM0_SUBMODULE2", 48u32),
    ("RESERVED65", 49u32),
    ("QDC0_COMPARE", 50u32),
    ("QDC0_HOME", 51u32),
    ("QDC0_WATCHDOG", 52u32),
    ("QDC0_INDEX", 53u32),
    ("FREQME0", 54u32),
    ("LPTMR0", 55u32),
    ("RESERVED72", 56u32),
    ("OS_EVENT", 57u32),
    ("WAKETIMER0", 58u32),
    ("UTICK0", 59u32),
    ("WWDT0", 60u32),
    ("RESERVED77", 61u32),
    ("ADC0", 62u32),
    ("ADC1", 63u32),
    ("CMP0", 64u32),
    ("CMP1", 65u32),
    ("RESERVED82", 66u32),
    ("DAC0", 67u32),
    ("RESERVED84", 68u32),
    ("RESERVED85", 69u32),
    ("RESERVED86", 70u32),
    ("GPIO0", 71u32),
    ("GPIO1", 72u32),
    ("GPIO2", 73u32),
    ("GPIO3", 74u32),
    ("GPIO4", 75u32),
    ("RESERVED92", 76u32),
    ("LPI2C2", 77u32),
    ("LPI2C3", 78u32),
    ("FLEXPWM1_RELOAD_ERROR", 79u32),
    ("FLEXPWM1_FAULT", 80u32),
    ("FLEXPWM1_SUBMODULE0", 81u32),
    ("FLEXPWM1_SUBMODULE1", 82u32),
    ("FLEXPWM1_SUBMODULE2", 83u32),
    ("RESERVED100", 84u32),
    ("QDC1_COMPARE", 85u32),
    ("QDC1_HOME", 86u32),
    ("QDC1_WATCHDOG", 87u32),
    ("QDC1_INDEX", 88u32),
];
