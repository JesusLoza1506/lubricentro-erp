#[derive(Debug, Clone, PartialEq)]
pub struct PermisoRolEntity {
    pub id_permiso: i32,
    pub rol: String,
    pub modulo: String,
    pub puede_ver: bool,
    pub puede_crear: bool,
    pub puede_editar: bool,
    pub puede_eliminar: bool,
}
