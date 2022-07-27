use super::super::c;
use bitflags::bitflags;

bitflags! {
    /// `FD_*` constants for use with [`fcntl_getfd`] and [`fcntl_setfd`].
    ///
    /// [`fcntl_getfd`]: crate::fs::fcntl_getfd
    /// [`fcntl_setfd`]: crate::fs::fcntl_setfd
    pub struct FdFlags: c::c_uint {
        /// `FD_CLOEXEC`
        const CLOEXEC = linux_raw_sys::general::FD_CLOEXEC;
    }
}

bitflags! {
    /// `*_OK` constants for use with [`accessat`].
    ///
    /// [`accessat`]: fn.accessat.html
    pub struct Access: c::c_uint {
        /// `R_OK`
        const READ_OK = linux_raw_sys::general::R_OK;

        /// `W_OK`
        const WRITE_OK = linux_raw_sys::general::W_OK;

        /// `X_OK`
        const EXEC_OK = linux_raw_sys::general::X_OK;

        /// `F_OK`
        const EXISTS = linux_raw_sys::general::F_OK;
    }
}

bitflags! {
    /// `AT_*` constants for use with [`openat`], [`statat`], and other `*at`
    /// functions.
    ///
    /// [`openat`]: crate::fs::openat
    /// [`statat`]: crate::fs::statat
    pub struct AtFlags: c::c_uint {
        /// `AT_REMOVEDIR`
        const REMOVEDIR = linux_raw_sys::general::AT_REMOVEDIR;

        /// `AT_SYMLINK_FOLLOW`
        const SYMLINK_FOLLOW = linux_raw_sys::general::AT_SYMLINK_FOLLOW;

        /// `AT_SYMLINK_NOFOLLOW`
        const SYMLINK_NOFOLLOW = linux_raw_sys::general::AT_SYMLINK_NOFOLLOW;

        /// `AT_EMPTY_PATH`
        const EMPTY_PATH = linux_raw_sys::general::AT_EMPTY_PATH;

        /// `AT_EACCESS`
        const EACCESS = linux_raw_sys::general::AT_EACCESS;

        /// `AT_STATX_SYNC_AS_STAT`
        const STATX_SYNC_AS_STAT = linux_raw_sys::general::AT_STATX_SYNC_AS_STAT;

        /// `AT_STATX_FORCE_SYNC`
        const STATX_FORCE_SYNC = linux_raw_sys::general::AT_STATX_FORCE_SYNC;

        /// `AT_STATX_DONT_SYNC`
        const STATX_DONT_SYNC = linux_raw_sys::general::AT_STATX_DONT_SYNC;
    }
}

bitflags! {
    /// `S_I*` constants for use with [`openat`], [`chmodat`], and [`fchmod`].
    ///
    /// [`openat`]: crate::fs::openat
    /// [`chmodat`]: crate::fs::chmodat
    /// [`fchmod`]: crate::fs::fchmod
    pub struct Mode: RawMode {
        /// `S_IRWXU`
        const RWXU = linux_raw_sys::general::S_IRWXU;

        /// `S_IRUSR`
        const RUSR = linux_raw_sys::general::S_IRUSR;

        /// `S_IWUSR`
        const WUSR = linux_raw_sys::general::S_IWUSR;

        /// `S_IXUSR`
        const XUSR = linux_raw_sys::general::S_IXUSR;

        /// `S_IRWXG`
        const RWXG = linux_raw_sys::general::S_IRWXG;

        /// `S_IRGRP`
        const RGRP = linux_raw_sys::general::S_IRGRP;

        /// `S_IWGRP`
        const WGRP = linux_raw_sys::general::S_IWGRP;

        /// `S_IXGRP`
        const XGRP = linux_raw_sys::general::S_IXGRP;

        /// `S_IRWXO`
        const RWXO = linux_raw_sys::general::S_IRWXO;

        /// `S_IROTH`
        const ROTH = linux_raw_sys::general::S_IROTH;

        /// `S_IWOTH`
        const WOTH = linux_raw_sys::general::S_IWOTH;

        /// `S_IXOTH`
        const XOTH = linux_raw_sys::general::S_IXOTH;

        /// `S_ISUID`
        const SUID = linux_raw_sys::general::S_ISUID;

        /// `S_ISGID`
        const SGID = linux_raw_sys::general::S_ISGID;

        /// `S_ISVTX`
        const SVTX = linux_raw_sys::general::S_ISVTX;
    }
}

impl Mode {
    /// Construct a `Mode` from the mode bits of the `st_mode` field of
    /// a `Stat`.
    #[inline]
    pub const fn from_raw_mode(st_mode: RawMode) -> Self {
        Self::from_bits_truncate(st_mode)
    }

    /// Construct an `st_mode` value from `Stat`.
    #[inline]
    pub const fn as_raw_mode(self) -> RawMode {
        self.bits()
    }
}

bitflags! {
    /// `O_*` constants for use with [`openat`].
    ///
    /// [`openat`]: crate::fs::openat
    pub struct OFlags: c::c_uint {
        /// `O_ACCMODE`
        const ACCMODE = linux_raw_sys::general::O_ACCMODE;

        /// Similar to `ACCMODE`, but just includes the read/write flags, and
        /// no other flags.
        ///
        /// Some implementations include `O_PATH` in `O_ACCMODE`, when
        /// sometimes we really just want the read/write bits. Caution is
        /// indicated, as the presence of `O_PATH` may mean that the read/write
        /// bits don't have their usual meaning.
        const RWMODE = linux_raw_sys::general::O_RDONLY |
                       linux_raw_sys::general::O_WRONLY |
                       linux_raw_sys::general::O_RDWR;

        /// `O_APPEND`
        const APPEND = linux_raw_sys::general::O_APPEND;

        /// `O_CREAT`
        #[doc(alias = "CREAT")]
        const CREATE = linux_raw_sys::general::O_CREAT;

        /// `O_DIRECTORY`
        const DIRECTORY = linux_raw_sys::general::O_DIRECTORY;

        /// `O_DSYNC`. Linux 2.6.32 only supports `O_SYNC`.
        const DSYNC = linux_raw_sys::general::O_SYNC;

        /// `O_EXCL`
        const EXCL = linux_raw_sys::general::O_EXCL;

        /// `O_FSYNC`. Linux 2.6.32 only supports `O_SYNC`.
        const FSYNC = linux_raw_sys::general::O_SYNC;

        /// `O_NOFOLLOW`
        const NOFOLLOW = linux_raw_sys::general::O_NOFOLLOW;

        /// `O_NONBLOCK`
        const NONBLOCK = linux_raw_sys::general::O_NONBLOCK;

        /// `O_RDONLY`
        const RDONLY = linux_raw_sys::general::O_RDONLY;

        /// `O_WRONLY`
        const WRONLY = linux_raw_sys::general::O_WRONLY;

        /// `O_RDWR`
        const RDWR = linux_raw_sys::general::O_RDWR;

        /// `O_NOCTTY`
        const NOCTTY = linux_raw_sys::general::O_NOCTTY;

        /// `O_RSYNC`. Linux 2.6.32 only supports `O_SYNC`.
        const RSYNC = linux_raw_sys::general::O_SYNC;

        /// `O_SYNC`
        const SYNC = linux_raw_sys::general::O_SYNC;

        /// `O_TRUNC`
        const TRUNC = linux_raw_sys::general::O_TRUNC;

        /// `O_PATH`
        const PATH = linux_raw_sys::general::O_PATH;

        /// `O_CLOEXEC`
        const CLOEXEC = linux_raw_sys::general::O_CLOEXEC;

        /// `O_TMPFILE`
        const TMPFILE = linux_raw_sys::general::O_TMPFILE;

        /// `O_NOATIME`
        const NOATIME = linux_raw_sys::general::O_NOATIME;
    }
}

bitflags! {
    /// `RESOLVE_*` constants for use with [`openat2`].
    ///
    /// [`openat2`]: crate::fs::openat2
    #[derive(Default)]
    pub struct ResolveFlags: u64 {
        /// `RESOLVE_NO_XDEV`
        const NO_XDEV = linux_raw_sys::general::RESOLVE_NO_XDEV as u64;

        /// `RESOLVE_NO_MAGICLINKS`
        const NO_MAGICLINKS = linux_raw_sys::general::RESOLVE_NO_MAGICLINKS as u64;

        /// `RESOLVE_NO_SYMLINKS`
        const NO_SYMLINKS = linux_raw_sys::general::RESOLVE_NO_SYMLINKS as u64;

        /// `RESOLVE_BENEATH`
        const BENEATH = linux_raw_sys::general::RESOLVE_BENEATH as u64;

        /// `RESOLVE_IN_ROOT`
        const IN_ROOT = linux_raw_sys::general::RESOLVE_IN_ROOT as u64;
    }
}

bitflags! {
    /// `RENAME_*` constants for use with [`renameat_with`].
    ///
    /// [`renameat_with`]: crate::fs::renameat_with
    pub struct RenameFlags: c::c_uint {
        /// `RENAME_EXCHANGE`
        const EXCHANGE = linux_raw_sys::general::RENAME_EXCHANGE;

        /// `RENAME_NOREPLACE`
        const NOREPLACE = linux_raw_sys::general::RENAME_NOREPLACE;

        /// `RENAME_WHITEOUT`
        const WHITEOUT = linux_raw_sys::general::RENAME_WHITEOUT;
    }
}

/// `S_IF*` constants for use with [`mknodat`] and [`Stat`]'s `st_mode` field.
///
/// [`mknodat`]: crate::fs::mknodat
/// [`Stat`]: crate::fs::Stat
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileType {
    /// `S_IFREG`
    RegularFile = linux_raw_sys::general::S_IFREG as isize,

    /// `S_IFDIR`
    Directory = linux_raw_sys::general::S_IFDIR as isize,

    /// `S_IFLNK`
    Symlink = linux_raw_sys::general::S_IFLNK as isize,

    /// `S_IFIFO`
    Fifo = linux_raw_sys::general::S_IFIFO as isize,

    /// `S_IFSOCK`
    Socket = linux_raw_sys::general::S_IFSOCK as isize,

    /// `S_IFCHR`
    CharacterDevice = linux_raw_sys::general::S_IFCHR as isize,

    /// `S_IFBLK`
    BlockDevice = linux_raw_sys::general::S_IFBLK as isize,

    /// An unknown filesystem object.
    Unknown,
}

impl FileType {
    /// Construct a `FileType` from the `S_IFMT` bits of the `st_mode` field of
    /// a `Stat`.
    #[inline]
    pub const fn from_raw_mode(st_mode: RawMode) -> Self {
        match st_mode & linux_raw_sys::general::S_IFMT {
            linux_raw_sys::general::S_IFREG => Self::RegularFile,
            linux_raw_sys::general::S_IFDIR => Self::Directory,
            linux_raw_sys::general::S_IFLNK => Self::Symlink,
            linux_raw_sys::general::S_IFIFO => Self::Fifo,
            linux_raw_sys::general::S_IFSOCK => Self::Socket,
            linux_raw_sys::general::S_IFCHR => Self::CharacterDevice,
            linux_raw_sys::general::S_IFBLK => Self::BlockDevice,
            _ => Self::Unknown,
        }
    }

    /// Construct an `st_mode` value from `Stat`.
    #[inline]
    pub const fn as_raw_mode(self) -> RawMode {
        match self {
            Self::RegularFile => linux_raw_sys::general::S_IFREG,
            Self::Directory => linux_raw_sys::general::S_IFDIR,
            Self::Symlink => linux_raw_sys::general::S_IFLNK,
            Self::Fifo => linux_raw_sys::general::S_IFIFO,
            Self::Socket => linux_raw_sys::general::S_IFSOCK,
            Self::CharacterDevice => linux_raw_sys::general::S_IFCHR,
            Self::BlockDevice => linux_raw_sys::general::S_IFBLK,
            Self::Unknown => linux_raw_sys::general::S_IFMT,
        }
    }

    /// Construct a `FileType` from the `d_type` field of a `dirent`.
    #[inline]
    pub(crate) const fn from_dirent_d_type(d_type: u8) -> Self {
        match d_type as u32 {
            linux_raw_sys::general::DT_REG => Self::RegularFile,
            linux_raw_sys::general::DT_DIR => Self::Directory,
            linux_raw_sys::general::DT_LNK => Self::Symlink,
            linux_raw_sys::general::DT_SOCK => Self::Socket,
            linux_raw_sys::general::DT_FIFO => Self::Fifo,
            linux_raw_sys::general::DT_CHR => Self::CharacterDevice,
            linux_raw_sys::general::DT_BLK => Self::BlockDevice,
            // linux_raw_sys::general::DT_UNKNOWN |
            _ => Self::Unknown,
        }
    }
}

/// `POSIX_FADV_*` constants for use with [`fadvise`].
///
/// [`fadvise`]: crate::fs::fadvise
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum Advice {
    /// `POSIX_FADV_NORMAL`
    Normal = linux_raw_sys::general::POSIX_FADV_NORMAL,

    /// `POSIX_FADV_SEQUENTIAL`
    Sequential = linux_raw_sys::general::POSIX_FADV_SEQUENTIAL,

    /// `POSIX_FADV_RANDOM`
    Random = linux_raw_sys::general::POSIX_FADV_RANDOM,

    /// `POSIX_FADV_NOREUSE`
    NoReuse = linux_raw_sys::general::POSIX_FADV_NOREUSE,

    /// `POSIX_FADV_WILLNEED`
    WillNeed = linux_raw_sys::general::POSIX_FADV_WILLNEED,

    /// `POSIX_FADV_DONTNEED`
    DontNeed = linux_raw_sys::general::POSIX_FADV_DONTNEED,
}

bitflags! {
    /// `MFD_*` constants for use with [`memfd_create`].
    ///
    /// [`memfd_create`]: crate::fs::memfd_create
    pub struct MemfdFlags: c::c_uint {
        /// `MFD_CLOEXEC`
        const CLOEXEC = linux_raw_sys::general::MFD_CLOEXEC;

        /// `MFD_ALLOW_SEALING`
        const ALLOW_SEALING = linux_raw_sys::general::MFD_ALLOW_SEALING;

        /// `MFD_HUGETLB` (since Linux 4.14)
        const HUGETLB = linux_raw_sys::general::MFD_HUGETLB;

        /// `MFD_HUGE_64KB`
        const HUGE_64KB = linux_raw_sys::general::MFD_HUGE_64KB;
        /// `MFD_HUGE_512JB`
        const HUGE_512KB = linux_raw_sys::general::MFD_HUGE_512KB;
        /// `MFD_HUGE_1MB`
        const HUGE_1MB = linux_raw_sys::general::MFD_HUGE_1MB;
        /// `MFD_HUGE_2MB`
        const HUGE_2MB = linux_raw_sys::general::MFD_HUGE_2MB;
        /// `MFD_HUGE_8MB`
        const HUGE_8MB = linux_raw_sys::general::MFD_HUGE_8MB;
        /// `MFD_HUGE_16MB`
        const HUGE_16MB = linux_raw_sys::general::MFD_HUGE_16MB;
        /// `MFD_HUGE_32MB`
        const HUGE_32MB = linux_raw_sys::general::MFD_HUGE_32MB;
        /// `MFD_HUGE_256MB`
        const HUGE_256MB = linux_raw_sys::general::MFD_HUGE_256MB;
        /// `MFD_HUGE_512MB`
        const HUGE_512MB = linux_raw_sys::general::MFD_HUGE_512MB;
        /// `MFD_HUGE_1GB`
        const HUGE_1GB = linux_raw_sys::general::MFD_HUGE_1GB;
        /// `MFD_HUGE_2GB`
        const HUGE_2GB = linux_raw_sys::general::MFD_HUGE_2GB;
        /// `MFD_HUGE_16GB`
        const HUGE_16GB = linux_raw_sys::general::MFD_HUGE_16GB;
    }
}

bitflags! {
    /// `F_SEAL_*` constants for use with [`fcntl_add_seals`] and
    /// [`fcntl_get_seals`].
    ///
    /// [`fcntl_add_seals`]: crate::fs::fcntl_add_seals
    /// [`fcntl_get_seals`]: crate::fs::fcntl_get_seals
    pub struct SealFlags: u32 {
       /// `F_SEAL_SEAL`.
       const SEAL = linux_raw_sys::general::F_SEAL_SEAL;
       /// `F_SEAL_SHRINK`.
       const SHRINK = linux_raw_sys::general::F_SEAL_SHRINK;
       /// `F_SEAL_GROW`.
       const GROW = linux_raw_sys::general::F_SEAL_GROW;
       /// `F_SEAL_WRITE`.
       const WRITE = linux_raw_sys::general::F_SEAL_WRITE;
       /// `F_SEAL_FUTURE_WRITE` (since Linux 5.1)
       const FUTURE_WRITE = linux_raw_sys::general::F_SEAL_FUTURE_WRITE;
    }
}

bitflags! {
    /// `STATX_*` constants for use with [`statx`].
    ///
    /// [`statx`]: crate::fs::statx
    pub struct StatxFlags: u32 {
        /// `STATX_TYPE`
        const TYPE = linux_raw_sys::general::STATX_TYPE;

        /// `STATX_MODE`
        const MODE = linux_raw_sys::general::STATX_MODE;

        /// `STATX_NLINK`
        const NLINK = linux_raw_sys::general::STATX_NLINK;

        /// `STATX_UID`
        const UID = linux_raw_sys::general::STATX_UID;

        /// `STATX_GID`
        const GID = linux_raw_sys::general::STATX_GID;

        /// `STATX_ATIME`
        const ATIME = linux_raw_sys::general::STATX_ATIME;

        /// `STATX_MTIME`
        const MTIME = linux_raw_sys::general::STATX_MTIME;

        /// `STATX_CTIME`
        const CTIME = linux_raw_sys::general::STATX_CTIME;

        /// `STATX_INO`
        const INO = linux_raw_sys::general::STATX_INO;

        /// `STATX_SIZE`
        const SIZE = linux_raw_sys::general::STATX_SIZE;

        /// `STATX_BLOCKS`
        const BLOCKS = linux_raw_sys::general::STATX_BLOCKS;

        /// `STATX_BASIC_STATS`
        const BASIC_STATS = linux_raw_sys::general::STATX_BASIC_STATS;

        /// `STATX_BTIME`
        const BTIME = linux_raw_sys::general::STATX_BTIME;

        /// `STATX_MNT_ID` (since Linux 5.8)
        const MNT_ID = linux_raw_sys::general::STATX_MNT_ID;

        /// `STATX_ALL`
        const ALL = linux_raw_sys::general::STATX_ALL;
    }
}

bitflags! {
    /// `FALLOC_FL_*` constants for use with [`fallocate`].
    ///
    /// [`fallocate`]: crate::fs::fallocate
    pub struct FallocateFlags: u32 {
        /// `FALLOC_FL_KEEP_SIZE`
        const KEEP_SIZE = linux_raw_sys::general::FALLOC_FL_KEEP_SIZE;
        /// `FALLOC_FL_PUNCH_HOLE`
        const PUNCH_HOLE = linux_raw_sys::general::FALLOC_FL_PUNCH_HOLE;
        /// `FALLOC_FL_NO_HIDE_STALE`
        const NO_HIDE_STALE = linux_raw_sys::general::FALLOC_FL_NO_HIDE_STALE;
        /// `FALLOC_FL_COLLAPSE_RANGE`
        const COLLAPSE_RANGE = linux_raw_sys::general::FALLOC_FL_COLLAPSE_RANGE;
        /// `FALLOC_FL_ZERO_RANGE`
        const ZERO_RANGE = linux_raw_sys::general::FALLOC_FL_ZERO_RANGE;
        /// `FALLOC_FL_INSERT_RANGE`
        const INSERT_RANGE = linux_raw_sys::general::FALLOC_FL_INSERT_RANGE;
        /// `FALLOC_FL_UNSHARE_RANGE`
        const UNSHARE_RANGE = linux_raw_sys::general::FALLOC_FL_UNSHARE_RANGE;
    }
}

/// `LOCK_*` constants for use with [`flock`]
///
/// [`flock`]: crate::fs::flock
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum FlockOperation {
    /// `LOCK_SH`
    LockShared = linux_raw_sys::general::LOCK_SH,
    /// `LOCK_EX`
    LockExclusive = linux_raw_sys::general::LOCK_EX,
    /// `LOCK_UN`
    Unlock = linux_raw_sys::general::LOCK_UN,
    /// `LOCK_SH | LOCK_NB`
    NonBlockingLockShared = linux_raw_sys::general::LOCK_SH | linux_raw_sys::general::LOCK_NB,
    /// `LOCK_EX | LOCK_NB`
    NonBlockingLockExclusive = linux_raw_sys::general::LOCK_EX | linux_raw_sys::general::LOCK_NB,
    /// `LOCK_UN | LOCK_NB`
    NonBlockingUnlock = linux_raw_sys::general::LOCK_UN | linux_raw_sys::general::LOCK_NB,
}

/// `struct stat` for use with [`statat`] and [`fstat`].
///
/// [`fstat`]: crate::fs::fstat
#[cfg(target_pointer_width = "32")]
pub type Stat = linux_raw_sys::general::stat64;

/// `struct stat` for use with [`statat`] and [`fstat`].
///
/// [`statat`]: crate::fs::statat
/// [`fstat`]: crate::fs::fstat
#[cfg(target_pointer_width = "64")]
pub type Stat = linux_raw_sys::general::stat;

/// `struct statfs` for use with [`fstatfs`].
///
/// [`fstatfs`]: crate::fs::fstatfs
#[cfg(target_pointer_width = "32")]
#[allow(clippy::module_name_repetitions)]
pub type StatFs = linux_raw_sys::general::statfs64;

/// `struct statfs` for use with [`fstatfs`].
///
/// [`fstatfs`]: crate::fs::fstatfs
#[cfg(target_pointer_width = "64")]
#[allow(clippy::module_name_repetitions)]
pub type StatFs = linux_raw_sys::general::statfs64;

/// `struct statx` for use with [`statx`].
///
/// [`statx`]: crate::fs::statx
pub type Statx = linux_raw_sys::general::statx;

/// `struct statx_timestamp` for use with [`Statx`].
pub type StatxTimestamp = linux_raw_sys::general::statx_timestamp;

/// `mode_t`
#[cfg(not(any(
    target_arch = "x86",
    target_arch = "sparc",
    target_arch = "avr",
    target_arch = "arm"
)))]
pub type RawMode = linux_raw_sys::general::__kernel_mode_t;

/// `mode_t
#[cfg(any(
    target_arch = "x86",
    target_arch = "sparc",
    target_arch = "avr",
    target_arch = "arm"
))]
// Don't use `__kernel_mode_t` since it's `u16` which differs from `st_size`.
pub type RawMode = c::c_uint;

/// `dev_t`
// Within the kernel the dev_t is 32-bit, but userspace uses a 64-bit field.
pub type Dev = u64;

/// `__fsword_t`
#[cfg(not(target_arch = "mips64"))]
pub type FsWord = linux_raw_sys::general::__fsword_t;

/// `__fsword_t`
#[cfg(target_arch = "mips64")]
pub type FsWord = i64;

pub use linux_raw_sys::general::{UTIME_NOW, UTIME_OMIT};

/// `PROC_SUPER_MAGIC`—The magic number for the procfs filesystem.
pub const PROC_SUPER_MAGIC: FsWord = linux_raw_sys::general::PROC_SUPER_MAGIC as FsWord;

/// `NFS_SUPER_MAGIC`—The magic number for the NFS filesystem.
pub const NFS_SUPER_MAGIC: FsWord = linux_raw_sys::general::NFS_SUPER_MAGIC as FsWord;
