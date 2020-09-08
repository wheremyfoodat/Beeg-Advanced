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