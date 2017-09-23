//! Interpreter OpCodes

/// OpCodes for the interpreter. 1..1024 are reserved for CPython opcodes
/// while 1024.. are for rsnek specific opcodes.
#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq, Serialize)]
pub enum OpCode {
    PopTop                   =   1,
    RotTwo                   =   2,
    RotThree                 =   3,
    DupTop                   =   4,
    DupTopTwo                =   5,
    Nop                      =   9,
    UnaryPositive            =  10,
    UnaryNegative            =  11,
    UnaryNot                 =  12,
    UnaryInvert              =  15,
    BinaryMatrixMultiply     =  16,
    InplaceMatrixMultiply    =  17,
    BinaryPower              =  19,
    BinaryMultiply           =  20,
    BinaryModulo             =  22,
    BinaryAdd                =  23,
    BinarySubtract           =  24,
    BinarySubscr             =  25,
    BinaryFloorDivide        =  26,
    BinaryTrueDivide         =  27,
    InplaceFloorDivide       =  28,
    InplaceTrueDivide        =  29,
    GetAiter                 =  50,
    GetAnext                 =  51,
    BeforeAsyncWith          =  52,
    InplaceAdd               =  55,
    InplaceSubtract          =  56,
    InplaceMultiply          =  57,
    InplaceModulo            =  59,
    StoreSubscr              =  60,
    DeleteSubscr             =  61,
    BinaryLshift             =  62,
    BinaryRshift             =  63,
    BinaryAnd                =  64,
    BinaryXor                =  65,
    BinaryOr                 =  66,
    InplacePower             =  67,
    GetIter                  =  68,
    GetYieldFromIter         =  69,
    PrintExpr                =  70,
    LoadBuildClass           =  71,
    YieldFrom                =  72,
    GetAwaitable             =  73,
    InplaceLshift            =  75,
    InplaceRshift            =  76,
    InplaceAnd               =  77,
    InplaceXor               =  78,
    InplaceOr                =  79,
    BreakLoop                =  80,
    WithCleanupStart         =  81,
    WithCleanupFinish        =  82,
    ReturnValue              =  83,
    ImportStar               =  84,
    YieldValue               =  86,
    PopBlock                 =  87,
    EndFinally               =  88,
    PopExcept                =  89,
    StoreName                =  90,
    DeleteName               =  91,
    UnpackSequence           =  92,
    ForIter                  =  93,
    UnpackEx                 =  94,
    StoreAttr                =  95,
    DeleteAttr               =  96,
    StoreGlobal              =  97,
    DeleteGlobal             =  98,
    LoadConst                = 100,
    LoadName                 = 101,
    BuildTuple               = 102,
    BuildList                = 103,
    BuildSet                 = 104,
    BuildMap                 = 105,
    LoadAttr                 = 106,
    CompareOp                = 107,
    ImportName               = 108,
    ImportFrom               = 109,
    JumpForward              = 110,
    JumpIfFalseOrPop         = 111,
    JumpIfTrueOrPop          = 112,
    JumpAbsolute             = 113,
    PopJumpIfFalse           = 114,
    PopJumpIfTrue            = 115,
    LoadGlobal               = 116,
    ContinueLoop             = 119,
    SetupLoop                = 120,
    SetupExcept              = 121,
    SetupFinally             = 122,
    LoadFast                 = 124,
    StoreFast                = 125,
    DeleteFast               = 126,
    RaiseVarargs             = 130,
    CallFunction             = 131,
    MakeFunction             = 132,
    BuildSlice               = 133,
    MakeClosure              = 134,
    LoadClosure              = 135,
    LoadDeref                = 136,
    StoreDeref               = 137,
    DeleteDeref              = 138,
    CallFunctionVar          = 140,
    CallFunctionKw           = 141,
    CallFunctionVarKw        = 142,
    SetupWith                = 143,
    ExtendedArg              = 144,
    ListAppend               = 145,
    SetAdd                   = 146,
    MapAdd                   = 147,
    LoadClassderef           = 148,
    BuildListUnpack          = 149,
    BuildMapUnpack           = 150,
    BuildMapUnpackWithCall   = 151,
    BuildTupleUnpack         = 152,
    BuildSetUnpack           = 153,
    SetupAsyncWith           = 154,

    // Defined for rsnek for now because the jump instructions are kinda weird without
    // frames and pointers to lines and stuff.
    LogicalAnd               = 1024,
    LogicalOr                = 1025,
    AssertCondition          = 1026,
    CompareEqual             = 1027,
    CompareNotEqual          = 1028,
    CompareLess              = 1029,
    CompareLessOrEqual       = 1030,
    CompareGreater           = 1031,
    CompareGreaterOrEqual    = 1032,
    CompareIn                = 1033,
    CompareNotIn             = 1034,
    CompareIs                = 1035,
    CompareIsNot             = 1036,

    SetLineNumber            = 2048,
}
