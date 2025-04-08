use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgPRT {
    pub port_id: u8,
    pub reserved0: u8,
    pub tx_ready: u16,
    pub mode: u32,
    pub baud_rate: u32,
    pub in_proto_mask: u16,
    pub out_proto_mask: u16,
    pub flags: u16,
    pub reserved1: u16,
}

impl CfgPRT {
    pub const CLASS_ID: u8 = 6;
    pub const MESSAGE_ID: u8 = 0;
    pub const PORT_ID_DDC: u8 = 0;
    pub const PORT_ID_UART1: u8 = 1;
    pub const PORT_ID_UART2: u8 = 2;
    pub const PORT_ID_USB: u8 = 3;
    pub const PORT_ID_SPI: u8 = 4;
    pub const TX_READY_EN: u16 = 1;
    pub const TX_READY_POLARITY_HIGH_ACTIVE: u16 = 0;
    pub const TX_READY_POLARITY_LOW_ACTIVE: u16 = 2;
    pub const TX_READY_PIN_SHIFT: u16 = 2;
    pub const TX_READY_PIN_MASK: u16 = 124;
    pub const TX_READY_THRES_SHIFT: u16 = 7;
    pub const TX_READY_THRES_MASK: u16 = 65408;
    pub const MODE_DDC_SLAVE_ADDR_SHIFT: u32 = 1;
    pub const MODE_DDC_SLAVE_ADDR_MASK: u32 = 254;
    pub const MODE_RESERVED1: u32 = 16;
    pub const MODE_CHAR_LEN_MASK: u32 = 192;
    pub const MODE_CHAR_LEN_5BIT: u32 = 0;
    pub const MODE_CHAR_LEN_6BIT: u32 = 64;
    pub const MODE_CHAR_LEN_7BIT: u32 = 128;
    pub const MODE_CHAR_LEN_8BIT: u32 = 192;
    pub const MODE_PARITY_MASK: u32 = 3584;
    pub const MODE_PARITY_EVEN: u32 = 0;
    pub const MODE_PARITY_ODD: u32 = 512;
    pub const MODE_PARITY_NO: u32 = 2048;
    pub const MODE_STOP_BITS_MASK: u32 = 12288;
    pub const MODE_STOP_BITS_1: u32 = 0;
    pub const MODE_STOP_BITS_15: u32 = 4096;
    pub const MODE_STOP_BITS_2: u32 = 8192;
    pub const MODE_STOP_BITS_05: u32 = 12288;
    pub const MODE_SPI_SPI_MODE_CPOL: u32 = 4;
    pub const MODE_SPI_SPI_MODE_CPHA: u32 = 2;
    pub const MODE_SPI_FLOW_CONTROL: u32 = 64;
    pub const MODE_SPI_FF_COUNT_SHIFT: u32 = 8;
    pub const MODE_SPI_FF_COUNT_MASK: u32 = 16128;
    pub const PROTO_UBX: u16 = 1;
    pub const PROTO_NMEA: u16 = 2;
    pub const PROTO_RTCM: u16 = 4;
    pub const PROTO_RTCM3: u16 = 32;
    pub const FLAGS_EXTENDED_TX_TIMEOUT: u16 = 2;
}

impl Default for CfgPRT {
    fn default() -> Self {
        CfgPRT {
            port_id: 0,
            reserved0: 0,
            tx_ready: 0,
            mode: 0,
            baud_rate: 0,
            in_proto_mask: 0,
            out_proto_mask: 0,
            flags: 0,
            reserved1: 0,
        }
    }
}

impl ros2_client::Message for CfgPRT {}
