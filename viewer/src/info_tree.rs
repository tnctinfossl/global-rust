use std::rc::Rc;
use std::sync::{Arc, RwLock};
use model::World;
pub struct InfoTree {
    tree_view: gtk::TreeView,
    world:Arc<RwLock<World>>
}

impl InfoTree {
    pub fn new(world:Arc<RwLock<World>>) -> Rc<InfoTree> {
        
        let tree_view = gtk::TreeViewBuilder::new().build();

        Rc::new(InfoTree {
            tree_view:tree_view,
            world:world
        })
    }
}
