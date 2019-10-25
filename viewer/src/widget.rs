pub trait AsWidget{
    fn as_widget_ref(&self)->&gtk::Widget;
}