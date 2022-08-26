
pub enum Context {
    Calculate,
    Verilog,
    VerilogNand,
    VerilogNor
}

impl TryFrom<String> for Context {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value {
            _ if value == "calculate" => Ok(Context::Calculate),
            _ if value == "verilog" => Ok(Context::Verilog),
            _ if value == "verilog nand" => Ok(Context::VerilogNand),
            _ if value == "verilog nor" => Ok(Context::VerilogNor),
            _ => Err(())
        }
    }

}

impl TryFrom<&str> for Context {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            _ if value == "calculate" => Ok(Context::Calculate),
            _ if value == "verilog" => Ok(Context::Verilog),
            _ if value == "verilog nand" => Ok(Context::VerilogNand),
            _ if value == "verilog nor" => Ok(Context::VerilogNor),
            _ => Err(())
        }
    }

}
