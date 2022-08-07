use super::ffi::*;
use once_cell::sync::Lazy;
use std::marker::PhantomData;

pub struct Cluster {
    pub id: u32,
}

pub struct Package {
    pub name: String,
    pub clusters: Vec<Cluster>,
}

struct Wrapper {
    pub packages: Vec<Package>,
    pub phantom: PhantomData<Vec<Package>>,
}

impl Drop for Wrapper {
    fn drop(&mut self) {
        unsafe { cpuinfo_deinitialize() };
    }
}

static GLOBAL_SOCKETS: Lazy<Wrapper> = Lazy::new(|| {
    unsafe { cpuinfo_initialize() };

    let mut sockets: Vec<Package> = vec![];

    let packages_count = unsafe { cpuinfo_get_packages_count() };

    let packages_ptr = unsafe { cpuinfo_get_packages() };

    let packages = unsafe { std::slice::from_raw_parts(packages_ptr, packages_count as usize) };

    for p in packages {
        let name = unsafe { std::ffi::CStr::from_ptr(p.name.as_ptr()) };

        let mut clusters = vec![];

        for cid in p.cluster_start..(p.cluster_start + p.cluster_count) {
            let maybe_cluster = unsafe { cpuinfo_get_cluster(cid).as_ref() };
            match maybe_cluster {
                Some(c) => {
                    let cluster = Cluster { id: c.cluster_id };

                    clusters.push(cluster);
                }
                None => {}
            }
        }

        let package = Package {
            name: name
                .to_owned()
                .to_str()
                .expect("failed to construct string")
                .to_owned(),
            clusters,
        };
        sockets.push(package);
    }

    return Wrapper {
        packages: sockets,
        phantom: PhantomData::default(),
    };
});

pub fn get_packages() -> &'static [Package] {
    return &GLOBAL_SOCKETS.packages;
}

#[cfg(test)]
mod test {
    use crate::get_packages;

    #[test]
    fn basic_info() {
        let packages = get_packages();
        assert!(!packages.is_empty());
        assert!(!packages.first().unwrap().name.is_empty());
    }
}
