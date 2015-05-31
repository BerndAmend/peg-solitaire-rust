use board::*;

pub struct English {
    desc: Description,
}

impl English {
    pub fn new() -> English {
        use board::MoveDirections::*;
        English {
            desc: Description::new("English", "..ooo..\n..ooo..\nooooooo\nooooooo\nooooooo\n..ooo..\n..ooo..", &[Horizontal, Vertical]).unwrap()
        }
    }
}

impl Board for English {
    fn description(&self) -> &Description {
        &self.desc
    }

    fn normalize(&self, state: State) -> State {
        let mut n = state;
        let p0 = (
               ((state & 56u64) << 24) | ((state & 1040384u64)) | ((state & 8128u64) << 14) | ((state & 7u64) << 30)
             | ((state & 7516192768u64) >> 30) | ((state & 939524096u64) >> 24) | ((state & 133169152u64) >> 14));
        if p0 < n { n = p0; }
        let p1 = (
               ((state & 67637248u64) >> 6) | ((state & 1056832u64) << 6) | ((state & 33818624u64) >> 4) | ((state & 2424373778u64))
             | ((state & 2113664u64) << 4) | ((state & 1212186889u64) << 2) | ((state & 4848747556u64) >> 2));
        if p1 < n { n = p1; }
        let p2 = (
               ((state & 4294967296u64) >> 32) | ((state & 4194304u64) >> 12) | ((state & 524288u64) >> 6) | ((state & 536870912u64) >> 26)
             | ((state & 64u64) << 20) | ((state & 8u64) << 26) | ((state & 33554432u64) >> 18) | ((state & 262144u64) >> 4)
             | ((state & 4u64) << 28) | ((state & 8388608u64) >> 14) | ((state & 8192u64) << 6) | ((state & 2048u64) << 10)
             | ((state & 1024u64) << 12) | ((state & 256u64) << 16) | ((state & 2u64) << 30) | ((state & 16384u64) << 4)
             | ((state & 1073741824u64) >> 28) | ((state & 128u64) << 18) | ((state & 16777216u64) >> 16) | ((state & 2147483648u64) >> 30)
             | ((state & 1u64) << 32) | ((state & 134217728u64) >> 22) | ((state & 65536u64)) | ((state & 512u64) << 14)
             | ((state & 32u64) << 22) | ((state & 32768u64) << 2) | ((state & 4096u64) << 8) | ((state & 268435456u64) >> 24)
             | ((state & 67108864u64) >> 20) | ((state & 2097152u64) >> 10) | ((state & 131072u64) >> 2) | ((state & 1048576u64) >> 8)
             | ((state & 16u64) << 24));
        if p2 < n { n = p2; }
        let p3 = (
               ((state & 4303388736u64) >> 6) | ((state & 262160u64) << 10) | ((state & 1074790400u64) >> 18) | ((state & 33554440u64) << 4)
             | ((state & 525314u64) << 12) | ((state & 2080u64) << 16) | ((state & 536871040u64) >> 4) | ((state & 268451840u64) >> 10)
             | ((state & 16843008u64)) | ((state & 67240449u64) << 6) | ((state & 2151686144u64) >> 12) | ((state & 4100u64) << 18)
             | ((state & 136314880u64) >> 16));
        if p3 < n { n = p3; }
        let p4 = (
               ((state & 524289u64) << 12) | ((state & 1040u64) << 14) | ((state & 2149580800u64) >> 18) | ((state & 131072u64) << 6)
             | ((state & 32768u64) >> 6) | ((state & 16384u64) >> 10) | ((state & 545259520u64) >> 8) | ((state & 4294975488u64) >> 12)
             | ((state & 272629760u64) >> 14) | ((state & 262144u64) << 10) | ((state & 16777344u64) >> 2) | ((state & 1073741824u64) >> 24)
             | ((state & 64u64) >> 4) | ((state & 4128u64) << 20) | ((state & 135266304u64) >> 20) | ((state & 33554688u64) << 2)
             | ((state & 2050u64) << 18) | ((state & 65536u64)) | ((state & 520u64) << 8) | ((state & 4u64) << 24)
             | ((state & 67108864u64) << 4));
        if p4 < n { n = p4; }
        let p5 = (
               ((state & 268435456u64) >> 10) | ((state & 134218752u64) >> 2) | ((state & 8200u64) << 18) | ((state & 129u64) << 20)
             | ((state & 512u64) << 6) | ((state & 16u64) << 10) | ((state & 65536u64)) | ((state & 8388608u64) >> 6)
             | ((state & 17039360u64) >> 14) | ((state & 537395200u64) >> 18) | ((state & 4194336u64) << 2) | ((state & 1048578u64) << 12)
             | ((state & 2147487744u64) >> 12) | ((state & 67108864u64) >> 24) | ((state & 133120u64) >> 8) | ((state & 16640u64) << 14)
             | ((state & 4328521728u64) >> 20) | ((state & 1073741824u64) >> 4) | ((state & 4u64) << 4) | ((state & 64u64) << 24)
             | ((state & 2129920u64) << 8));
        if p5 < n { n = p5; }
        let p6 = (
               ((state & 8519680u64) >> 8) | ((state & 16777216u64) >> 16) | ((state & 4260864u64)) | ((state & 65u64) << 26)
             | ((state & 8194u64) << 18) | ((state & 134219776u64) >> 6) | ((state & 2148007936u64) >> 18) | ((state & 1073745920u64) >> 10)
             | ((state & 16400u64) << 14) | ((state & 570425344u64) >> 22) | ((state & 4362076160u64) >> 26) | ((state & 136u64) << 22)
             | ((state & 1048580u64) << 10) | ((state & 268697600u64) >> 14) | ((state & 33280u64) << 8) | ((state & 256u64) << 16)
             | ((state & 2097184u64) << 6));
        if p6 < n { n = p6; }
        n
    }

    fn equivalent_fields(&self, state: State) -> [State; 8] {
        let mut n = [EMPTY_STATE; 8];
        n[0] = state;
        n[1] = (
               ((state & 56u64) << 24) | ((state & 1040384u64)) | ((state & 8128u64) << 14) | ((state & 7u64) << 30)
             | ((state & 7516192768u64) >> 30) | ((state & 939524096u64) >> 24) | ((state & 133169152u64) >> 14));
        n[2] = (
               ((state & 67637248u64) >> 6) | ((state & 1056832u64) << 6) | ((state & 33818624u64) >> 4) | ((state & 2424373778u64))
             | ((state & 2113664u64) << 4) | ((state & 1212186889u64) << 2) | ((state & 4848747556u64) >> 2));
        n[3] = (
               ((state & 4294967296u64) >> 32) | ((state & 4194304u64) >> 12) | ((state & 524288u64) >> 6) | ((state & 536870912u64) >> 26)
             | ((state & 64u64) << 20) | ((state & 8u64) << 26) | ((state & 33554432u64) >> 18) | ((state & 262144u64) >> 4)
             | ((state & 4u64) << 28) | ((state & 8388608u64) >> 14) | ((state & 8192u64) << 6) | ((state & 2048u64) << 10)
             | ((state & 1024u64) << 12) | ((state & 256u64) << 16) | ((state & 2u64) << 30) | ((state & 16384u64) << 4)
             | ((state & 1073741824u64) >> 28) | ((state & 128u64) << 18) | ((state & 16777216u64) >> 16) | ((state & 2147483648u64) >> 30)
             | ((state & 1u64) << 32) | ((state & 134217728u64) >> 22) | ((state & 65536u64)) | ((state & 512u64) << 14)
             | ((state & 32u64) << 22) | ((state & 32768u64) << 2) | ((state & 4096u64) << 8) | ((state & 268435456u64) >> 24)
             | ((state & 67108864u64) >> 20) | ((state & 2097152u64) >> 10) | ((state & 131072u64) >> 2) | ((state & 1048576u64) >> 8)
             | ((state & 16u64) << 24));
        n[4] = (
               ((state & 4303388736u64) >> 6) | ((state & 262160u64) << 10) | ((state & 1074790400u64) >> 18) | ((state & 33554440u64) << 4)
             | ((state & 525314u64) << 12) | ((state & 2080u64) << 16) | ((state & 536871040u64) >> 4) | ((state & 268451840u64) >> 10)
             | ((state & 16843008u64)) | ((state & 67240449u64) << 6) | ((state & 2151686144u64) >> 12) | ((state & 4100u64) << 18)
             | ((state & 136314880u64) >> 16));
        n[5] = (
               ((state & 524289u64) << 12) | ((state & 1040u64) << 14) | ((state & 2149580800u64) >> 18) | ((state & 131072u64) << 6)
             | ((state & 32768u64) >> 6) | ((state & 16384u64) >> 10) | ((state & 545259520u64) >> 8) | ((state & 4294975488u64) >> 12)
             | ((state & 272629760u64) >> 14) | ((state & 262144u64) << 10) | ((state & 16777344u64) >> 2) | ((state & 1073741824u64) >> 24)
             | ((state & 64u64) >> 4) | ((state & 4128u64) << 20) | ((state & 135266304u64) >> 20) | ((state & 33554688u64) << 2)
             | ((state & 2050u64) << 18) | ((state & 65536u64)) | ((state & 520u64) << 8) | ((state & 4u64) << 24)
             | ((state & 67108864u64) << 4));
        n[6] = (
               ((state & 268435456u64) >> 10) | ((state & 134218752u64) >> 2) | ((state & 8200u64) << 18) | ((state & 129u64) << 20)
             | ((state & 512u64) << 6) | ((state & 16u64) << 10) | ((state & 65536u64)) | ((state & 8388608u64) >> 6)
             | ((state & 17039360u64) >> 14) | ((state & 537395200u64) >> 18) | ((state & 4194336u64) << 2) | ((state & 1048578u64) << 12)
             | ((state & 2147487744u64) >> 12) | ((state & 67108864u64) >> 24) | ((state & 133120u64) >> 8) | ((state & 16640u64) << 14)
             | ((state & 4328521728u64) >> 20) | ((state & 1073741824u64) >> 4) | ((state & 4u64) << 4) | ((state & 64u64) << 24)
             | ((state & 2129920u64) << 8));
        n[7] = (
               ((state & 8519680u64) >> 8) | ((state & 16777216u64) >> 16) | ((state & 4260864u64)) | ((state & 65u64) << 26)
             | ((state & 8194u64) << 18) | ((state & 134219776u64) >> 6) | ((state & 2148007936u64) >> 18) | ((state & 1073745920u64) >> 10)
             | ((state & 16400u64) << 14) | ((state & 570425344u64) >> 22) | ((state & 4362076160u64) >> 26) | ((state & 136u64) << 22)
             | ((state & 1048580u64) << 10) | ((state & 268697600u64) >> 14) | ((state & 33280u64) << 8) | ((state & 256u64) << 16)
             | ((state & 2097184u64) << 6));
        n
    }
}

pub struct European {
    desc: Description,
}

impl European {
    pub fn new() -> European {
        use board::MoveDirections::*;
        European {
            desc: Description::new("European", "..ooo..\n.ooooo.\nooooooo\nooooooo\nooooooo\n.ooooo.\n..ooo..", &[Horizontal, Vertical]).unwrap()
        }
    }
}

impl Board for European {
    fn description(&self) -> &Description {
        &self.desc
    }

    fn normalize(&self, state: State) -> State {
        let mut n = state;
        let p0 = (
               ((state & 16642998272u64) >> 26) | ((state & 120259084288u64) >> 34) | ((state & 532676608u64) >> 14) | ((state & 7u64) << 34)
             | ((state & 4161536u64)) | ((state & 32512u64) << 14) | ((state & 248u64) << 26));
        if p0 < n { n = p0; }
        let p1 = (
               ((state & 36541040674u64)) | ((state & 545325576u64) << 4) | ((state & 18270520337u64) << 2) | ((state & 73082081348u64) >> 2)
             | ((state & 270548992u64) >> 6) | ((state & 8725209216u64) >> 4) | ((state & 4227328u64) << 6));
        if p1 < n { n = p1; }
        let p2 = (
               ((state & 268435456u64) >> 20) | ((state & 1048576u64) >> 4) | ((state & 16777216u64) >> 12) | ((state & 2097152u64) >> 6)
             | ((state & 2u64) << 34) | ((state & 4194304u64) >> 8) | ((state & 32768u64) << 6) | ((state & 64u64) << 24)
             | ((state & 16u64) << 28) | ((state & 4294967296u64) >> 28) | ((state & 524288u64) >> 2) | ((state & 8192u64) << 10)
             | ((state & 536870912u64) >> 22) | ((state & 1024u64) << 16) | ((state & 2048u64) << 14) | ((state & 256u64) << 20)
             | ((state & 1u64) << 36) | ((state & 4096u64) << 12) | ((state & 1073741824u64) >> 24) | ((state & 2147483648u64) >> 26)
             | ((state & 33554432u64) >> 14) | ((state & 68719476736u64) >> 36) | ((state & 8589934592u64) >> 30) | ((state & 8u64) << 30)
             | ((state & 34359738368u64) >> 34) | ((state & 17179869184u64) >> 32) | ((state & 67108864u64) >> 16) | ((state & 262144u64))
             | ((state & 128u64) << 22) | ((state & 32u64) << 26) | ((state & 512u64) << 18) | ((state & 4u64) << 32)
             | ((state & 131072u64) << 2) | ((state & 16384u64) << 8) | ((state & 8388608u64) >> 10) | ((state & 65536u64) << 4)
             | ((state & 134217728u64) >> 18));
        if p2 < n { n = p2; }
        let p3 = (
               ((state & 68719476992u64) >> 8) | ((state & 34359771136u64) >> 14) | ((state & 1082130432u64) >> 17) | ((state & 268435457u64) << 8)
             | ((state & 2147549184u64) >> 11) | ((state & 8657306632u64)) | ((state & 128u64) << 22) | ((state & 16777216u64) >> 12)
             | ((state & 2097154u64) << 14) | ((state & 17184063488u64) >> 20) | ((state & 16388u64) << 20) | ((state & 536870912u64) >> 22)
             | ((state & 4294967808u64) >> 5) | ((state & 4096u64) << 12) | ((state & 1048608u64) << 11) | ((state & 33685504u64) >> 6)
             | ((state & 526336u64) << 6) | ((state & 134217744u64) << 5) | ((state & 8256u64) << 17));
        if p3 < n { n = p3; }
        let p4 = (
               ((state & 67108864u64) >> 2) | ((state & 16u64) << 9) | ((state & 34359738368u64) >> 20) | ((state & 8589934592u64) >> 4)
             | ((state & 8388608u64) >> 19) | ((state & 134217728u64) << 3) | ((state & 8u64) << 4) | ((state & 2147483648u64) >> 15)
             | ((state & 8192u64) << 19) | ((state & 1048576u64) << 11) | ((state & 64u64) << 21) | ((state & 262144u64))
             | ((state & 131328u64) >> 6) | ((state & 512u64) >> 3) | ((state & 132u64) << 26) | ((state & 1073741824u64) >> 21)
             | ((state & 2u64) << 20) | ((state & 4194304u64) >> 22) | ((state & 33554432u64) >> 8) | ((state & 17716740096u64) >> 26)
             | ((state & 68736286720u64) >> 14) | ((state & 16384u64) << 22) | ((state & 2048u64) << 8) | ((state & 65536u64) >> 11)
             | ((state & 1024u64) << 2) | ((state & 268959744u64) << 6) | ((state & 2101249u64) << 14) | ((state & 32u64) << 15)
             | ((state & 4294967296u64) >> 9));
        if p4 < n { n = p4; }
        let p5 = (
               ((state & 16777216u64) << 2) | ((state & 32768u64) << 20) | ((state & 4096u64) >> 2) | ((state & 8388608u64) << 9)
             | ((state & 536870912u64) << 4) | ((state & 131072u64) << 8) | ((state & 16u64) << 19) | ((state & 1048576u64) >> 15)
             | ((state & 64u64) << 3) | ((state & 128u64) >> 4) | ((state & 32u64) << 11) | ((state & 1u64) << 22)
             | ((state & 8858370048u64) >> 26) | ((state & 2147483648u64) >> 11) | ((state & 68719476736u64) >> 22) | ((state & 524288u64) >> 8)
             | ((state & 17213423616u64) >> 6) | ((state & 2097152u64) >> 20) | ((state & 4294967296u64) >> 19) | ((state & 512u64) << 21)
             | ((state & 65536u64) << 15) | ((state & 134217728u64) >> 21) | ((state & 262144u64)) | ((state & 8192u64) >> 9)
             | ((state & 34426863616u64) >> 14) | ((state & 2052u64) << 6) | ((state & 1073741824u64) >> 3) | ((state & 4195330u64) << 14)
             | ((state & 264u64) << 26));
        if p5 < n { n = p5; }
        let p6 = (
               ((state & 67108864u64) >> 16) | ((state & 65568u64) << 15) | ((state & 8u64) << 30) | ((state & 8388672u64) << 7)
             | ((state & 528u64) << 23) | ((state & 133120u64) << 8) | ((state & 68987912192u64) >> 28) | ((state & 2148532224u64) >> 15)
             | ((state & 1073750016u64) >> 7) | ((state & 1024u64) << 16) | ((state & 32770u64) << 20) | ((state & 17179885568u64) >> 12)
             | ((state & 8589934592u64) >> 30) | ((state & 34078720u64) >> 8) | ((state & 257u64) << 28) | ((state & 4429185024u64) >> 23)
             | ((state & 34361835520u64) >> 20) | ((state & 4194308u64) << 12) | ((state & 553914496u64)));
        if p6 < n { n = p6; }
        n
    }

    fn equivalent_fields(&self, state: State) -> [State; 8] {
        let mut n = [EMPTY_STATE; 8];
        n[0] = state;
        n[1] = (
               ((state & 16642998272u64) >> 26) | ((state & 120259084288u64) >> 34) | ((state & 532676608u64) >> 14) | ((state & 7u64) << 34)
             | ((state & 4161536u64)) | ((state & 32512u64) << 14) | ((state & 248u64) << 26));
        n[2] = (
               ((state & 36541040674u64)) | ((state & 545325576u64) << 4) | ((state & 18270520337u64) << 2) | ((state & 73082081348u64) >> 2)
             | ((state & 270548992u64) >> 6) | ((state & 8725209216u64) >> 4) | ((state & 4227328u64) << 6));
        n[3] = (
               ((state & 268435456u64) >> 20) | ((state & 1048576u64) >> 4) | ((state & 16777216u64) >> 12) | ((state & 2097152u64) >> 6)
             | ((state & 2u64) << 34) | ((state & 4194304u64) >> 8) | ((state & 32768u64) << 6) | ((state & 64u64) << 24)
             | ((state & 16u64) << 28) | ((state & 4294967296u64) >> 28) | ((state & 524288u64) >> 2) | ((state & 8192u64) << 10)
             | ((state & 536870912u64) >> 22) | ((state & 1024u64) << 16) | ((state & 2048u64) << 14) | ((state & 256u64) << 20)
             | ((state & 1u64) << 36) | ((state & 4096u64) << 12) | ((state & 1073741824u64) >> 24) | ((state & 2147483648u64) >> 26)
             | ((state & 33554432u64) >> 14) | ((state & 68719476736u64) >> 36) | ((state & 8589934592u64) >> 30) | ((state & 8u64) << 30)
             | ((state & 34359738368u64) >> 34) | ((state & 17179869184u64) >> 32) | ((state & 67108864u64) >> 16) | ((state & 262144u64))
             | ((state & 128u64) << 22) | ((state & 32u64) << 26) | ((state & 512u64) << 18) | ((state & 4u64) << 32)
             | ((state & 131072u64) << 2) | ((state & 16384u64) << 8) | ((state & 8388608u64) >> 10) | ((state & 65536u64) << 4)
             | ((state & 134217728u64) >> 18));
        n[4] = (
               ((state & 68719476992u64) >> 8) | ((state & 34359771136u64) >> 14) | ((state & 1082130432u64) >> 17) | ((state & 268435457u64) << 8)
             | ((state & 2147549184u64) >> 11) | ((state & 8657306632u64)) | ((state & 128u64) << 22) | ((state & 16777216u64) >> 12)
             | ((state & 2097154u64) << 14) | ((state & 17184063488u64) >> 20) | ((state & 16388u64) << 20) | ((state & 536870912u64) >> 22)
             | ((state & 4294967808u64) >> 5) | ((state & 4096u64) << 12) | ((state & 1048608u64) << 11) | ((state & 33685504u64) >> 6)
             | ((state & 526336u64) << 6) | ((state & 134217744u64) << 5) | ((state & 8256u64) << 17));
        n[5] = (
               ((state & 67108864u64) >> 2) | ((state & 16u64) << 9) | ((state & 34359738368u64) >> 20) | ((state & 8589934592u64) >> 4)
             | ((state & 8388608u64) >> 19) | ((state & 134217728u64) << 3) | ((state & 8u64) << 4) | ((state & 2147483648u64) >> 15)
             | ((state & 8192u64) << 19) | ((state & 1048576u64) << 11) | ((state & 64u64) << 21) | ((state & 262144u64))
             | ((state & 131328u64) >> 6) | ((state & 512u64) >> 3) | ((state & 132u64) << 26) | ((state & 1073741824u64) >> 21)
             | ((state & 2u64) << 20) | ((state & 4194304u64) >> 22) | ((state & 33554432u64) >> 8) | ((state & 17716740096u64) >> 26)
             | ((state & 68736286720u64) >> 14) | ((state & 16384u64) << 22) | ((state & 2048u64) << 8) | ((state & 65536u64) >> 11)
             | ((state & 1024u64) << 2) | ((state & 268959744u64) << 6) | ((state & 2101249u64) << 14) | ((state & 32u64) << 15)
             | ((state & 4294967296u64) >> 9));
        n[6] = (
               ((state & 16777216u64) << 2) | ((state & 32768u64) << 20) | ((state & 4096u64) >> 2) | ((state & 8388608u64) << 9)
             | ((state & 536870912u64) << 4) | ((state & 131072u64) << 8) | ((state & 16u64) << 19) | ((state & 1048576u64) >> 15)
             | ((state & 64u64) << 3) | ((state & 128u64) >> 4) | ((state & 32u64) << 11) | ((state & 1u64) << 22)
             | ((state & 8858370048u64) >> 26) | ((state & 2147483648u64) >> 11) | ((state & 68719476736u64) >> 22) | ((state & 524288u64) >> 8)
             | ((state & 17213423616u64) >> 6) | ((state & 2097152u64) >> 20) | ((state & 4294967296u64) >> 19) | ((state & 512u64) << 21)
             | ((state & 65536u64) << 15) | ((state & 134217728u64) >> 21) | ((state & 262144u64)) | ((state & 8192u64) >> 9)
             | ((state & 34426863616u64) >> 14) | ((state & 2052u64) << 6) | ((state & 1073741824u64) >> 3) | ((state & 4195330u64) << 14)
             | ((state & 264u64) << 26));
        n[7] = (
               ((state & 67108864u64) >> 16) | ((state & 65568u64) << 15) | ((state & 8u64) << 30) | ((state & 8388672u64) << 7)
             | ((state & 528u64) << 23) | ((state & 133120u64) << 8) | ((state & 68987912192u64) >> 28) | ((state & 2148532224u64) >> 15)
             | ((state & 1073750016u64) >> 7) | ((state & 1024u64) << 16) | ((state & 32770u64) << 20) | ((state & 17179885568u64) >> 12)
             | ((state & 8589934592u64) >> 30) | ((state & 34078720u64) >> 8) | ((state & 257u64) << 28) | ((state & 4429185024u64) >> 23)
             | ((state & 34361835520u64) >> 20) | ((state & 4194308u64) << 12) | ((state & 553914496u64)));
        n
    }
}

pub struct Holes15 {
    desc: Description,
}

impl Holes15 {
    pub fn new() -> Holes15 {
        use board::MoveDirections::*;
        Holes15 {
            desc: Description::new("Holes15", "o....\noo...\nooo..\noooo.\nooooo", &[Horizontal, Vertical, LeftDiagonal, RightDiagonal]).unwrap()
        }
    }
}

impl Board for Holes15 {
    fn description(&self) -> &Description {
        &self.desc
    }

    fn normalize(&self, state: State) -> State {
        let mut n = state;
        let p0 = (
               ((state & 2u64) << 12) | ((state & 2048u64) >> 9) | ((state & 32u64) << 7) | ((state & 8u64) << 5)
             | ((state & 4u64) << 9) | ((state & 1u64) << 14) | ((state & 256u64) >> 5) | ((state & 656u64))
             | ((state & 4096u64) >> 7) | ((state & 16384u64) >> 14) | ((state & 1024u64) >> 4) | ((state & 8192u64) >> 12)
             | ((state & 64u64) << 4));
        if p0 < n { n = p0; }
        n
    }

    fn equivalent_fields(&self, state: State) -> [State; 8] {
        let mut n = [EMPTY_STATE; 8];
        n[0] = state;
        n[1] = (
               ((state & 2u64) << 12) | ((state & 2048u64) >> 9) | ((state & 32u64) << 7) | ((state & 8u64) << 5)
             | ((state & 4u64) << 9) | ((state & 1u64) << 14) | ((state & 256u64) >> 5) | ((state & 656u64))
             | ((state & 4096u64) >> 7) | ((state & 16384u64) >> 14) | ((state & 1024u64) >> 4) | ((state & 8192u64) >> 12)
             | ((state & 64u64) << 4));
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use board::Board;

    #[test]
    fn test_english_board() {
        let board = English::new();
        board.description().verify_board(&board);
    }

    #[test]
    fn test_european_board() {
        let board = European::new();
        board.description().verify_board(&board);
    }
    
    #[test]
    fn test_hole15_board() {
        let board = Holes15::new();
        board.description().verify_board(&board);
    }
}