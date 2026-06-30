use crate::*; use ketypes::*;

Ku!
{
    KeFsLookup              or  |_,_,_|     None

    KeFsReadDir             or  |_,_,_|     None

    KeFsRead                or  |_,_,_,_|   Err(KeFsError::Unknown)

    KeFsWrite               or  |_,_,_,_|   Err(KeFsError::Unknown)

    KeFsTrucate             or  |_,_,_|     Err(KeFsError::Unknown)

    KeFsLink                or  |_,_,_,_|   Err(KeFsError::Unknown)

    KeFsUnlink              or  |_,_,_|     Err(KeFsError::Unknown)

    KeFsObjectNew           or  |_,_,_|     Err(KeFsError::Unknown)

    KeFsObjectStat          or  |_,_|       None

    KeFsObjectIsMountPoint  or  |_|         false

    KeFsResolve             or  |_|         Err(KeFsError::Unknown)

    KeFsListDir             or  |_,_|       BTreeMap::new()

    KeFsReadToString        or  |_,_|       Err(KeFsError::Unknown)

    KeFsMetaBlockRegister   or  |_|         0

    KeFsMount               or  |_,_|       None

    KeFsCurrentRoot         or  ||          "".to_string()
}
