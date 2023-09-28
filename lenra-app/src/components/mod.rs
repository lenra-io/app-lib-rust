use crate::manifest;

pub mod json;
pub mod lenra;

impl Into<manifest::DefsProps> for lenra::DefsProps {
    fn into(self) -> manifest::DefsProps {
        manifest::DefsProps(self.0)
    }
}

impl Into<manifest::DataQuery> for lenra::DataQuery {
    fn into(self) -> manifest::DataQuery {
        manifest::DataQuery(self.0)
    }
}

impl Into<manifest::DataProjection> for lenra::DataProjection {
    fn into(self) -> manifest::DataProjection {
        manifest::DataProjection(self.0)
    }
}

impl Into<manifest::ComponentsViewDefinitionsFind> for lenra::ViewDefinitionsFind {
    fn into(self) -> manifest::ComponentsViewDefinitionsFind {
        manifest::ComponentsViewDefinitionsFind {
            coll: self.coll,
            query: self.query.into(),
            projection: self.projection.map(|projection| projection.into()),
        }
    }
}

impl Into<manifest::ComponentsView> for lenra::builder::View {
    fn into(self) -> manifest::ComponentsView {
        let view: lenra::View = self.try_into().unwrap();
        manifest::ComponentsView {
            context: view.context,
            find: view.find.map(|find| find.into()),
            name: view.name,
            props: view.props.map(|props| props.into()),
            type_: view.type_,
        }
    }
}
