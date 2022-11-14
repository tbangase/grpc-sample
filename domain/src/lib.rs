use common::external_libs::{derive_getters::Getters, derive_new::new};

#[derive(Debug, new, Getters)]
pub struct DomainModel {
    id: i32,
    name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lib_test() {
        let id = 1;
        let name = "name".to_string();

        let domain_model = DomainModel::new(1, "name".to_string());
        assert!(domain_model.id() == &id);
        assert!(domain_model.name() == &name);
    }
}
