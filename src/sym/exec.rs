use crate::*; use ketypes::*;

Ku!
{
    KeExecPanic             or   |_|            { error!("KeExecPanic not provided"); loop{core::hint::spin_loop()} }

    KeExecExit              or   |_|            { error!("KeExecExit not provided"); loop{core::hint::spin_loop()} }

    KeExecYield             or   ||             { error!("KeExecYield not provided"); }

    KeExecSleep             or   |_|            { error!("KeExecSleep not provided"); }

    KeExecSpawn             or   |_,_,_,_,_|    { error!("KeExecSpawn not provided"); 0 }

    KeExecArgumentedSpawn   or   |_,_,_,_,_,_|  { error!("KeExecArgumentedSpawn not provided"); 0 }

    KeExecWaitChild         or   |_|            { error!("KeExecWaitChild not provided"); -1 }

    KeExecSetDeadline       or   |_,_|          { error!("KeExecSetDeadline not provided"); }
}
