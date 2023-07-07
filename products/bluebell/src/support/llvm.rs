trait LllvmSupport {
    fn get_type_definition(&self, name: &str) -> Result<BasicTypeEnum<'ctx>, String>;
}
