use bitfield::*;

bitfield!{
    pub struct DISPSTAT(u16);
    pub getRaw,              setRaw:              15, 0;
    pub getVBlankFlag,       setVBlankFlag:       0, 0;
    pub getHBlankFlag,       setHBlankFlag:       1, 1;
    pub getCoincidenceFlag,  setCoincidenceFlag:  2, 2; // LY == LYC flag
    pub getVBlankIRQEnable,  setVBlankIRQEnable:  3, 3;
    pub getHBlankIRQEnable,  setHBlankIRQEnable:  4, 4;
    pub getLYCIRQEnable,     setLYCIRQEnable:     5, 5;
    pub getLYC,              setLYC:              15, 8;
}

bitfield!{
    pub struct KEYINPUT(u16);
    pub getRaw, setRaw: 15, 0;
    pub _, setA: 0, 0;
    pub _, setB: 1, 1;
    pub _, setSelect: 2, 2;
    pub _, setStart: 3, 3;
    pub _, setRight: 4, 4;
    pub _, setLeft: 5, 5;
    pub _, setUp: 6, 6;
    pub _, setDown: 7, 7;
    pub _, setR: 8, 8;
    pub _, setL: 9, 9;
}