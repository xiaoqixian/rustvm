pub mod binary_file_parser;

pub mod byte_code {
    pub const POP_TOP:u8 = 1;
    pub const ROT_TWO:u8 = 2;
    pub const ROT_THREE:u8 = 3;
    pub const DUP_TOP:u8 = 4;
    pub const UNARY_NEGATIVE:u8 = 11;
    pub const BINARY_MULTIPLY:u8 = 20;
    pub const BINARY_MODULO:u8 = 22;
    pub const BINARY_SUBSCR:u8 = 25;
    pub const BINARY_DIVIDE:u8 = 21;
    pub const BINARY_ADD:u8 = 23;
    pub const BINARY_SUBTRACT:u8 = 24;

    pub const INPLACE_ADD:u8 = 55;
    pub const STORE_MAP:u8 = 54;
    pub const INPLACE_SUBSTRACT:u8 = 56;
    pub const INPLACE_MULTIPLY:u8 = 57;
    pub const INPLACE_DIVIDE:u8 = 58;
    pub const INPLACE_MODULO:u8 = 59;
    pub const STORE_SUBSCR:u8 = 60;
    pub const DELETE_SUBSCR:u8 = 61;

    pub const GET_ITER:u8 = 68;

    pub const PRINT_ITEM:u8 = 71;
    pub const PRINT_NEWLINE:u8 = 72;

    pub const BREAK_LOOP:u8 = 80;
    pub const LOAD_LOCALS:u8 = 82;
    pub const RETURN_VALUE:u8 = 83;
    pub const YIELD_VALUE:u8 = 86;
    pub const POP_BLOCK:u8 = 87;
    pub const END_FINALLY:u8 = 88;
    pub const BUILD_CLASS:u8 = 89;

    // TODO: This is a separator
    pub const HAVE_ARGUMENT:u8 = 90; /* Opcodes from here have an argument: */

    pub const STORE_NAME:u8 = 90; /* Index in name list */
    pub const UNPACK_SEQUENCE:u8 = 92;
    pub const FOR_ITER:u8 = 93;
    pub const STORE_ATTR:u8 = 95;  /* Index in name list */
    pub const STORE_GLOBAL:u8 = 97;
    pub const DUP_TOPX:u8 = 99;   /* number of items to duplicate */
    pub const LOAD_CONST:u8 = 100; /* Index in const list */
    pub const LOAD_NAME:u8 = 101; /* Index in name list */
    pub const BUILD_TUPLE:u8 = 102;
    pub const BUILD_LIST:u8 = 103;
    pub const BUILD_MAP:u8 = 105;
    pub const LOAD_ATTR:u8 = 106; /* Index in name list */
    pub const COMPARE_OP:u8 = 107; /* Comparison operator */
    pub const IMPORT_NAME:u8 = 108; /* Index in name list */
    pub const IMPORT_FROM:u8 = 109; /* Index in name list */
    pub const JUMP_FORWARD:u8 = 110; /* Number of bytes to skip */
    pub const JUMP_IF_FALSE_OR_POP:u8 = 111; /* Target byte offset from beginning
                                    of code */

    pub const JUMP_ABSOLUTE:u8 = 113;
    pub const POP_JUMP_IF_FALSE:u8 = 114;
    pub const POP_JUMP_IF_TRUE:u8 = 115;
    pub const LOAD_GLOBAL:u8 = 116; /* Index in name list */

    pub const CONTINUE_LOOP:u8 = 119; /* Start of loop (absolute) */
    pub const SETUP_LOOP:u8 = 120; /* Target address (relative) */
    pub const SETUP_EXCEPT:u8 = 121;  /* "" */
    pub const SETUP_FINALLY:u8 = 122; /* "" */

    pub const LOAD_FAST :u8 = 124; /* Local variable number */
    pub const STORE_FAST:u8 = 125; /* Local variable number */

    pub const RAISE_VARARGS:u8 = 130;
    pub const CALL_FUNCTION:u8 = 131;
    pub const MAKE_FUNCTION:u8 = 132;

    pub const MAKE_CLOSURE:u8 = 134; /* #free vars */
    pub const LOAD_CLOSURE:u8 = 135; /* Load free variable from closure */
    pub const LOAD_DEREF:u8 = 136; /* Load and dereference from closure cell */
    pub const STORE_DEREF:u8 = 137; /* Store into cell */

    pub const CALL_FUNCTION_VAR:u8 = 140;

    pub enum COMPARE {
        LESS = 0,
        LessEqual,
        Equal,
        NotEqual,
        Greater,
        GreaterEqual,
        In,
        NotIn,
        Is,
        IsNot,
        ExcMatch
    }
}

#[derive(Debug)]
pub enum Ops {
    PopTop,
    RotTwo,
    RotThree,
    DupTop,
    UnaryNegative,
    BinaryMultiply,
    BinaryModulo,
    BinarySubscr,
    BinaryDivide,
    BinaryAdd,
    BinarySubtract,

    InplaceAdd,
    StoreMap,
    InplaceSubstract,
    InplaceMultiply,
    InplaceDivide,
    InplaceModulo,
    StoreSubscr,
    DeleteSubscr,

    GetIter,

    PrintItem,
    PrintNewline,

    BreakLoop,
    LoadLocals,
    ReturnValue,
    YieldValue,
    PopBlock,
    EndFinally,
    BuildClass,

    // TODO: This is a separator
    HaveArgument,

    StoreName,
    UnpackSequence,
    ForIter,
    StoreAttr,
    StoreGlobal,
    DupTopx,
    LoadConst,
    LoadName,
    BuildTuple,
    BuildList,
    BuildMap,
    LoadAttr,
    CompareOp,
    ImportName,
    ImportFrom,
    JumpForward,
    JumpIfFalseOrPop,

    JumpAbsolute,
    PopJumpIfFalse,
    PopJumpIfTrue,
    LoadGlobal,

    ContinueLoop,
    SetupLoop,
    SetupExcept,
    SetupFinally,

    LoadFast,
    StoreFast,

    RaiseVarargs,
    CallFunction,
    MakeFunction,

    MakeClosure,
    LoadClosure,
    LoadDeref,
    StoreDeref,

    CallFunctionVar,

}

pub fn get_op(index: u8) -> Option<Ops> {
    match index {
        1 => Some(Ops::PopTop),
        2 => Some(Ops::RotTwo),
        3 => Some(Ops::RotThree),
        4 => Some(Ops::DupTop),
        11 => Some(Ops::UnaryNegative),
        20 => Some(Ops::BinaryMultiply),
        22 => Some(Ops::BinaryModulo),
        25 => Some(Ops::BinarySubscr),
        21 => Some(Ops::BinaryDivide),
        23 => Some(Ops::BinaryAdd),
        24 => Some(Ops::BinarySubtract),

        55 => Some(Ops::InplaceAdd),
        54 => Some(Ops::StoreMap),
        56 => Some(Ops::InplaceSubstract),
        57 => Some(Ops::InplaceMultiply),
        58 => Some(Ops::InplaceDivide),
        59 => Some(Ops::InplaceModulo),
        60 => Some(Ops::StoreSubscr),
        61 => Some(Ops::DeleteSubscr),

        68 => Some(Ops::GetIter),

        71 => Some(Ops::PrintItem),
        72 => Some(Ops::PrintNewline),

        80 => Some(Ops::BreakLoop),
        82 => Some(Ops::LoadLocals),
        83 => Some(Ops::ReturnValue),
        86 => Some(Ops::YieldValue),
        87 => Some(Ops::PopBlock),
        88 => Some(Ops::EndFinally),
        89 => Some(Ops::BuildClass),

        // This is a separator
        90 => Some(Ops::HaveArgument), /* Opcodes from here have an argument: */

        90 => Some(Ops::StoreName), /* Index in name list */
        92 => Some(Ops::UnpackSequence),
        93 => Some(Ops::ForIter),
        95 => Some(Ops::StoreAttr),  /* Index in name list */
        97 => Some(Ops::StoreGlobal),
        99 => Some(Ops::DupTopx),   /* number of items to duplicate */
        100 => Some(Ops::LoadConst), /* Index in const list */
        101 => Some(Ops::LoadName), /* Index in name list */
        102 => Some(Ops::BuildTuple),
        103 => Some(Ops::BuildList),
        105 => Some(Ops::BuildMap),
        106 => Some(Ops::LoadAttr), /* Index in name list */
        107 => Some(Ops::CompareOp), /* Comparison operator */
        108 => Some(Ops::ImportName), /* Index in name list */
        109 => Some(Ops::ImportFrom), /* Index in name list */
        110 => Some(Ops::JumpForward), /* Number of bytes to skip */
        111 => Some(Ops::JumpIfFalseOrPop), /* Target byte offset from beginning
                                        of code */

        113 => Some(Ops::JumpAbsolute),
        114 => Some(Ops::PopJumpIfFalse),
        115 => Some(Ops::PopJumpIfTrue),
        116 => Some(Ops::LoadGlobal), /* Index in name list */

        119 => Some(Ops::ContinueLoop), /* Start of loop (absolute) */
        120 => Some(Ops::SetupLoop), /* Target address (relative) */
        121 => Some(Ops::SetupExcept),  /* "" */
        122 => Some(Ops::SetupFinally), /* "" */

        124 => Some(Ops::LoadFast ), /* Local variable number */
        125 => Some(Ops::StoreFast), /* Local variable number */

        130 => Some(Ops::RaiseVarargs),
        131 => Some(Ops::CallFunction),
        132 => Some(Ops::MakeFunction),

        134 => Some(Ops::MakeClosure), /* #free vars */
        135 => Some(Ops::LoadClosure), /* Load free variable from closure */
        136 => Some(Ops::LoadDeref), /* Load and dereference from closure cell */
        137 => Some(Ops::StoreDeref), /* Store into cell */

        140 => Some(Ops::CallFunctionVar),

        _ => None
    }
}
