use crate::DepMode;
use nfc1_sys::size_t;

/// Safe version of nfc_iso14443a_info
/// NFC ISO14443A tag (MIFARE) information
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Iso14443a {
    pub atqa: [u8; 2],
    pub sak: u8,
    pub uid: [u8; 10],
    pub uid_len: usize,
    pub ats: [u8; 254],
    pub ats_len: usize,
}

/// Safe version of nfc_felica_info
/// NFC FeLiCa tag information
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Felica {
    pub len: usize,
    pub res_code: u8,
    pub id: [u8; 8],
    pub pad: [u8; 8],
    pub sys_code: [u8; 2],
}

/// Safe version of nfc_iso14443b_info
/// NFC ISO14443B tag information
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Iso14443b {
    /// pupi stores PUPI contained in ATQB (Answer To reQuest of type B) (see ISO14443-3)
    pub pupi: [u8; 4],
    /// application_data stores Application Data contained in ATQB (see ISO14443-3)
    pub application_data: [u8; 4],
    /// protocol_info stores Protocol Info contained in ATQB (see ISO14443-3)
    pub protocol_info: [u8; 3],
    /// card_identifier stores CID (Card Identifier) attributted by PCD to the PICC
    pub card_identifier: u8,
}

/// Safe version of nfc_iso14443bi_info
/// NFC ISO14443B' tag information
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Iso14443bi {
    /// div: 4 LSBytes of tag serial number
    pub div: [u8; 4],
    /// Software version & type of REPGEN
    pub ver_log: u8,
    /// Config Byte, present if long REPGEN
    pub config: u8,
    /// ATR, if any
    pub atr: [u8; 33],
    pub atr_len: usize,
}

/// Safe version of nfc_iso14443b2sr_info
/// NFC ISO14443-2B ST SRx tag information
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Iso14443b2sr {
    pub uid: [u8; 8],
}

/// Safe version of nfc_iso14443b2ct_info
/// NFC ISO14443-2B ASK CTx tag information
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Iso14443b2ct {
    pub uid: [u8; 4],
    pub prod_code: u8,
    pub fab_code: u8,
}

/// Safe version of nfc_jewel_info
/// NFC Jewel tag information
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Jewel {
    pub sens_res: [u8; 2],
    pub id: [u8; 4],
}

/// Safe version of nfc_dep_info
/// NFC target information in D.E.P. (Data Exchange Protocol) see ISO/IEC 18092 (NFCIP-1)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Dep {
    /// NFCID3
    pub nfcid3: [u8; 10],
    /// DID
    pub did: u8,
    /// Supported send-bit rate
    pub bs: u8,
    /// Supported receive-bit rate
    pub br: u8,
    /// Timeout value
    pub to: u8,
    /// PP Parameters
    pub pp: u8,
    /// General Bytes
    pub gb: [u8; 48],
    pub gb_len: usize,
    /// DEP mode
    pub dep_mode: DepMode,
}

/// Safe version of nfc_barcode_info
/// Thinfilm NFC Barcode information
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Barcode {
    pub data: [u8; 32],
    pub data_len: usize,
}

/// Safe version of nfc_iso14443biclass_info
/// NFC ISO14443BiClass tag information
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Iso14443biClass {
    pub uid: [u8; 8],
}

/// Safe version of nfc_target_info
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TargetInfo {
    Iso14443a(Iso14443a),
    Felica(Felica),
    Iso14443b(Iso14443b),
    Iso14443bi(Iso14443bi),
    Iso14443b2sr(Iso14443b2sr),
    Iso14443b2ct(Iso14443b2ct),
    Jewel(Jewel),
    Dep(Dep),
    Barcode(Barcode),
    Iso14443biClass(Iso14443biClass),
}

impl From<&Iso14443a> for nfc1_sys::nfc_iso14443a_info {
    fn from(info: &Iso14443a) -> Self {
        Self {
            abtAtqa: info.atqa,
            btSak: info.sak,
            szUidLen: info.uid_len as size_t,
            abtUid: info.uid,
            szAtsLen: info.ats_len as size_t,
            abtAts: info.ats,
        }
    }
}

impl From<&Felica> for nfc1_sys::nfc_felica_info {
    fn from(info: &Felica) -> Self {
        Self {
            szLen: info.len as size_t,
            btResCode: info.res_code,
            abtId: info.id,
            abtPad: info.pad,
            abtSysCode: info.sys_code,
        }
    }
}

impl From<&Iso14443b> for nfc1_sys::nfc_iso14443b_info {
    fn from(info: &Iso14443b) -> Self {
        Self {
            abtPupi: info.pupi,
            abtApplicationData: info.application_data,
            abtProtocolInfo: info.protocol_info,
            ui8CardIdentifier: info.card_identifier,
        }
    }
}

impl From<&Iso14443bi> for nfc1_sys::nfc_iso14443bi_info {
    fn from(info: &Iso14443bi) -> Self {
        Self {
            abtDIV: info.div,
            btVerLog: info.ver_log,
            btConfig: info.config,
            szAtrLen: info.atr_len as size_t,
            abtAtr: info.atr,
        }
    }
}

impl From<&Iso14443b2sr> for nfc1_sys::nfc_iso14443b2sr_info {
    fn from(info: &Iso14443b2sr) -> Self {
        Self { abtUID: info.uid }
    }
}

impl From<&Iso14443b2ct> for nfc1_sys::nfc_iso14443b2ct_info {
    fn from(info: &Iso14443b2ct) -> Self {
        Self {
            abtUID: info.uid,
            btProdCode: info.prod_code,
            btFabCode: info.fab_code,
        }
    }
}

impl From<&Jewel> for nfc1_sys::nfc_jewel_info {
    fn from(info: &Jewel) -> Self {
        Self {
            btSensRes: info.sens_res,
            btId: info.id,
        }
    }
}

impl From<&Dep> for nfc1_sys::nfc_dep_info {
    fn from(info: &Dep) -> Self {
        Self {
            abtNFCID3: info.nfcid3,
            btDID: info.did,
            btBS: info.bs,
            btBR: info.br,
            btTO: info.to,
            btPP: info.pp,
            abtGB: info.gb,
            szGB: if info.gb_len > 48 {
                48
            } else {
                info.gb_len as size_t
            },
            ndm: info.dep_mode.into(),
        }
    }
}

impl From<&Barcode> for nfc1_sys::nfc_barcode_info {
    fn from(info: &Barcode) -> Self {
        Self {
            szDataLen: info.data_len as size_t,
            abtData: info.data,
        }
    }
}

impl From<&Iso14443biClass> for nfc1_sys::nfc_iso14443biclass_info {
    fn from(info: &Iso14443biClass) -> Self {
        Self { abtUID: info.uid }
    }
}

impl From<&TargetInfo> for nfc1_sys::nfc_target_info {
    fn from(input: &TargetInfo) -> nfc1_sys::nfc_target_info {
        match input {
            TargetInfo::Iso14443a(info) => nfc1_sys::nfc_target_info { nai: info.into() },
            TargetInfo::Felica(info) => nfc1_sys::nfc_target_info { nfi: info.into() },
            TargetInfo::Iso14443b(info) => nfc1_sys::nfc_target_info { nbi: info.into() },
            TargetInfo::Iso14443bi(info) => nfc1_sys::nfc_target_info { nii: info.into() },
            TargetInfo::Iso14443b2sr(info) => nfc1_sys::nfc_target_info { nsi: info.into() },
            TargetInfo::Iso14443b2ct(info) => nfc1_sys::nfc_target_info { nci: info.into() },
            TargetInfo::Jewel(info) => nfc1_sys::nfc_target_info { nji: info.into() },
            TargetInfo::Dep(info) => nfc1_sys::nfc_target_info { ndi: info.into() },
            TargetInfo::Barcode(info) => nfc1_sys::nfc_target_info { nti: info.into() },
            TargetInfo::Iso14443biClass(info) => nfc1_sys::nfc_target_info { nhi: info.into() },
        }
    }
}

impl TargetInfo {
    pub fn from_iso14443a(input: nfc1_sys::nfc_target_info) -> Self {
        unsafe {
            Self::Iso14443a(Iso14443a {
                atqa: input.nai.abtAtqa,
                sak: input.nai.btSak,
                uid_len: input.nai.szUidLen as usize,
                uid: input.nai.abtUid,
                ats_len: input.nai.szAtsLen as usize,
                ats: input.nai.abtAts,
            })
        }
    }

    pub fn from_felica(input: nfc1_sys::nfc_target_info) -> Self {
        unsafe {
            Self::Felica(Felica {
                len: input.nfi.szLen as usize,
                res_code: input.nfi.btResCode,
                id: input.nfi.abtId,
                pad: input.nfi.abtPad,
                sys_code: input.nfi.abtSysCode,
            })
        }
    }

    pub fn from_iso14443b(input: nfc1_sys::nfc_target_info) -> Self {
        unsafe {
            Self::Iso14443b(Iso14443b {
                pupi: input.nbi.abtPupi,
                application_data: input.nbi.abtApplicationData,
                protocol_info: input.nbi.abtProtocolInfo,
                card_identifier: input.nbi.ui8CardIdentifier,
            })
        }
    }

    pub fn from_iso14443bi(input: nfc1_sys::nfc_target_info) -> Self {
        unsafe {
            Self::Iso14443bi(Iso14443bi {
                div: input.nii.abtDIV,
                ver_log: input.nii.btVerLog,
                config: input.nii.btConfig,
                atr_len: input.nii.szAtrLen as usize,
                atr: input.nii.abtAtr,
            })
        }
    }

    pub fn from_iso14443b2sr(input: nfc1_sys::nfc_target_info) -> Self {
        unsafe {
            Self::Iso14443b2sr(Iso14443b2sr {
                uid: input.nsi.abtUID,
            })
        }
    }

    pub fn from_iso14443b2ct(input: nfc1_sys::nfc_target_info) -> Self {
        unsafe {
            Self::Iso14443b2ct(Iso14443b2ct {
                uid: input.nci.abtUID,
                prod_code: input.nci.btProdCode,
                fab_code: input.nci.btFabCode,
            })
        }
    }

    pub fn from_jewel(input: nfc1_sys::nfc_target_info) -> Self {
        unsafe {
            Self::Jewel(Jewel {
                sens_res: input.nji.btSensRes,
                id: input.nji.btId,
            })
        }
    }

    pub fn from_dep(input: nfc1_sys::nfc_target_info) -> Self {
        unsafe {
            Self::Dep(Dep {
                nfcid3: input.ndi.abtNFCID3,
                did: input.ndi.btDID,
                bs: input.ndi.btBS,
                br: input.ndi.btBR,
                to: input.ndi.btTO,
                pp: input.ndi.btPP,
                gb: input.ndi.abtGB,
                gb_len: input.ndi.szGB as usize,
                dep_mode: input.ndi.ndm.into(),
            })
        }
    }

    pub fn from_barcode(input: nfc1_sys::nfc_target_info) -> Self {
        unsafe {
            Self::Barcode(Barcode {
                data: input.nti.abtData,
                data_len: input.nti.szDataLen as usize,
            })
        }
    }

    pub fn from_iso14443biclass(input: nfc1_sys::nfc_target_info) -> Self {
        unsafe {
            Self::Iso14443biClass(Iso14443biClass {
                uid: input.nhi.abtUID,
            })
        }
    }

    pub fn new_iso14443a() -> Self {
        Self::Iso14443a(Iso14443a {
            atqa: [0u8; 2],
            sak: 0,
            uid: [0u8; 10],
            uid_len: 0,
            ats: [0u8; 254],
            ats_len: 0,
        })
    }

    pub fn new_felica() -> Self {
        Self::Felica(Felica {
            len: 0,
            res_code: 0,
            id: [0u8; 8],
            pad: [0u8; 8],
            sys_code: [0u8; 2],
        })
    }

    pub fn new_iso14443b() -> Self {
        Self::Iso14443b(Iso14443b {
            pupi: [0u8; 4],
            application_data: [0u8; 4],
            protocol_info: [0u8; 3],
            card_identifier: 0,
        })
    }

    pub fn new_iso14443bi() -> Self {
        Self::Iso14443bi(Iso14443bi {
            div: [0u8; 4],
            ver_log: 0,
            config: 0,
            atr: [0u8; 33],
            atr_len: 0,
        })
    }

    pub fn new_iso14443b2sr() -> Self {
        Self::Iso14443b2sr(Iso14443b2sr { uid: [0u8; 8] })
    }

    pub fn new_iso14443b2ct() -> Self {
        Self::Iso14443b2ct(Iso14443b2ct {
            uid: [0u8; 4],
            prod_code: 0,
            fab_code: 0,
        })
    }

    pub fn new_jewel() -> Self {
        Self::Jewel(Jewel {
            sens_res: [0u8; 2],
            id: [0u8; 4],
        })
    }

    pub fn new_dep() -> Self {
        Self::Dep(Dep {
            nfcid3: [0u8; 10],
            did: 0,
            bs: 0,
            br: 0,
            to: 0,
            pp: 0,
            gb: [0u8; 48],
            gb_len: 0,
            dep_mode: DepMode::Undefined,
        })
    }

    pub fn new_barcode() -> Self {
        Self::Barcode(Barcode {
            data: [0u8; 32],
            data_len: 0,
        })
    }

    pub fn new_iso14443biclass() -> Self {
        Self::Iso14443biClass(Iso14443biClass { uid: [0u8; 8] })
    }
}
