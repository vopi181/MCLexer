mod defines {
    #[derive(Debug)]
    pub enum cells_defines {
       variable,
       function,
    }
     #[derive(Debug)]
    pub struct cells_struct {
        cell_type: cells_defines,
        data: string
    }
}