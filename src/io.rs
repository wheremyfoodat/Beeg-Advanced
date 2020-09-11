use bitfield::*;
// TODO: Remove unneeded getters/setters

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
    pub struct DISPCNT(u16);
    pub getRaw,                setRaw:   15, 0;
    pub getMode,               _:        2, 0;
    pub getFrameSelect,        _:        4, 4;
    pub getHBlankIntervalFree, _:        5, 5;
    pub getOBJVRAMMapping,     _:        6, 6; 
    pub getForcedBlank,        _:        7, 7;
    pub getBGEnableBits,       _:        11, 8;
    pub getOBJEnable,          _:        12, 12;
    pub getWindowEnableBits,   _:        14, 13;
    pub getOBJWindowEnable,    _:        15, 15;
}

bitfield!{
    pub struct BGCNT(u16);
    pub getRaw,                 setRaw:   15, 0;
    pub getPriority,            _:        1, 0;
    pub getTileDataBase,        _:        3, 2;
    pub getMosaic,              _:        6, 6; 
    pub getBitDepth,            _:        7, 7;
    pub getMapDataBase,         _:        12, 8;
    pub getDisplayAreaOverflow, _:        13, 13; // For BG 2 and 3. 0 = transparent, 1 = wraparound
    pub getSize,                _:        15, 14;
}

bitfield!{
    pub struct BGOFS(u16);
    pub getRaw,    setRaw:   15, 0;
    pub getOffset, _     :    8, 0;                  
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