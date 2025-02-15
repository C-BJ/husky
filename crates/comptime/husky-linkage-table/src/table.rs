use std::path::Path;

use crate::*;
use __husky::init::__StaticLinkageKey;
use husky_vm::__Linkage;
use libloading::Library;
use smallvec::SmallVec;

#[derive(Clone)]
pub struct LinkageTable {
    internal: ASafeRwLock<Option<LinkageTableInternal>>,
    pub(crate) config: LinkageTableConfig,
}

pub struct LinkageTableInternal {
    linkages: HashMap<LinkageKey, __Linkage>,
    library: Library,
}

impl std::fmt::Debug for LinkageTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
        // self.internal.read(|internal| {
        //     f.debug_struct("LinkageTable")
        //         .field("linkages", internal.map(|internal|internal.linkages)
        //         .field("config", &self.config)
        //         .finish()
        // })
    }
}

impl LinkageTableInternal {
    fn new(db: &dyn ResolveLinkage, package_dir: &Path) -> Self {
        let library = get_library(package_dir).unwrap();
        let linkages_from_cdylib: &[(__StaticLinkageKey, __Linkage)] = unsafe {
            library
                .get::<GetLinkagesFromCDylib>(b"get_linkages")
                .expect("what")()
        };
        let linkages: HashMap<LinkageKey, __Linkage> = linkages_from_cdylib
            .iter()
            .map(|(static_key, linkage)| {
                let key = LinkageKey::from_static(db, *static_key);
                (key, *linkage)
            })
            .collect();
        Self { library, linkages }
    }
}

type GetLinkagesFromCDylib = unsafe extern "C" fn() -> &'static [(__StaticLinkageKey, __Linkage)];

fn get_library(package_dir: &Path) -> Option<Library> {
    use convert_case::*;
    #[cfg(target_os = "linux")]
    static DYLIB_EXTENSION: &'static str = "so";
    #[cfg(target_os = "macos")]
    static DYLIB_EXTENSION: &'static str = "dylib";
    let package_name = package_dir
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .with_boundaries(&[Boundary::Hyphen])
        .to_case(Case::Snake);
    let library_release_path = package_dir.join(format!(
        "__rust_gen__/target/release/lib{}.{DYLIB_EXTENSION}",
        package_name
    ));
    if library_release_path.exists() {
        return Some(unsafe { Library::new(library_release_path) }.expect("it should work"));
    }
    let library_debug_path = package_dir.join(format!(
        "__rust_gen__/target/debug/lib{}.{DYLIB_EXTENSION}",
        package_name,
    ));
    if library_debug_path.exists() {
        todo!()
    }
    None
}

impl LinkageTable {
    pub fn new(config: LinkageTableConfig) -> Self {
        Self {
            internal: Default::default(),
            config,
        }
    }

    pub fn load(&self, db: &dyn ResolveLinkage, package_dir: &Path) {
        self.internal
            .write(|internal| *internal = Some(LinkageTableInternal::new(db, package_dir)))
    }

    pub(crate) fn type_call_linkage(&self, ty_uid: EntityUid) -> Option<__Linkage> {
        self.get_linkage(LinkageKey::TypeCall { ty_uid })
    }

    pub(crate) fn feature_eager_block_linkage(&self, feature_uid: EntityUid) -> Option<__Linkage> {
        self.get_linkage(LinkageKey::FeatureEagerBlock { uid: feature_uid })
    }

    pub(crate) fn routine_linkage(&self, routine_uid: EntityUid) -> Option<__Linkage> {
        self.get_linkage(LinkageKey::Routine { routine_uid })
    }

    pub(crate) fn field_linkage_source(
        &self,
        this_ty_uid: EntityUid,
        field_ident: CustomIdentifier,
    ) -> Option<__Linkage> {
        self.get_linkage(LinkageKey::StructField {
            this_ty_uid,
            field_ident,
        })
    }

    pub(crate) fn element_access(&self, opd_uids: SmallVec<[EntityUid; 2]>) -> Option<__Linkage> {
        self.get_linkage(LinkageKey::Index { opd_uids })
    }

    fn get_linkage(&self, key: LinkageKey) -> Option<__Linkage> {
        self.internal.read(|entries| {
            entries
                .as_ref()
                .unwrap()
                .linkages
                .get(&key)
                .map(|linkage_source| *linkage_source)
        })
    }
}
