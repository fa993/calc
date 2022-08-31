use std::fmt::Write;
use std::sync::atomic::Ordering;
use std::{
    collections::{HashMap, HashSet},
    sync::{atomic::AtomicUsize, Arc},
};

use super::{
    funcs::{assemble_map_calc, assemble_map_veri, assemble_map_veri_nand, assemble_map_veri_nor},
    node::{CalcFunctionData, CalcNode, CalcUserFunctionData},
    EvalFunction,
};

#[derive(Clone, Copy)]
pub enum ContextType {
    Calculate,
    Verilog,
    VerilogNand,
    VerilogNor,
}

impl Default for ContextType {
    fn default() -> Self {
        ContextType::Calculate
    }
}

impl TryFrom<String> for ContextType {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value {
            _ if value == "calculate" => Ok(ContextType::Calculate),
            _ if value == "verilog" => Ok(ContextType::Verilog),
            _ if value == "verilog nand" => Ok(ContextType::VerilogNand),
            _ if value == "verilog nor" => Ok(ContextType::VerilogNor),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for ContextType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            _ if value == "calculate" => Ok(ContextType::Calculate),
            _ if value == "verilog" => Ok(ContextType::Verilog),
            _ if value == "verilog nand" => Ok(ContextType::VerilogNand),
            _ if value == "verilog nor" => Ok(ContextType::VerilogNor),
            _ => Err(()),
        }
    }
}

#[derive(Default)]
pub struct Context {
    pub built_in: HashMap<String, EvalFunction>,
    pub user_def: HashMap<String, CalcUserFunctionData>,
    pub specific: ContextType,
}

impl Context {
    pub fn print_result(&self, ans: &CalcNode) {
        match self.specific {
            ContextType::Calculate => {
                println!("{:?}", ans);
                // println!("{}", ans);
                // println!("{:#}", ans);
            }
            ContextType::Verilog | ContextType::VerilogNand | ContextType::VerilogNor => {
                // println!("{:?}", i);
                // println!("{:#?}", ans);
                println!("{}", self.to_verilog(ans));
            }
        }

        // todo!()
    }

    fn to_verilog(&self, node: &CalcNode) -> String {
        let mut t = String::new();
        let mut seen = HashSet::new();
        let mut infos = HashMap::new();
        self.to_verilog__(node, &mut t, &mut seen, &mut infos);

        print!("wire ");
        for (pos, t) in seen.iter().enumerate() {
            print!("w_{}", t);
            if pos != seen.len() - 1 {
                print!(", ");
            } else {
                println!(";");
            }
        }

        return t;
    }

    fn to_verilog__(
        &self,
        node: &CalcNode,
        sout: &mut String,
        seen: &mut HashSet<usize>,
        info: &mut HashMap<usize, CalcFunctionData>,
    ) {
        match node {
            CalcNode::Function(x) => {
                x.params
                    .iter()
                    .for_each(|y| self.to_verilog__(y, sout, seen, info));

                if !seen.insert(x.id) {
                    return;
                }

                if x.params.len() == 2 {
                    write!(
                        sout,
                        "{}({}, {}, {});\n",
                        x.name,
                        format!("w_{}", x.id),
                        if let CalcNode::Text(y) = &x.params[0] {
                            format!("{}", y)
                        } else if let CalcNode::Function(y) = &x.params[0] {
                            format!("w_{}", y.id)
                        } else {
                            panic!("Type not allowed")
                        },
                        if let CalcNode::Text(y) = &x.params[1] {
                            format!("{}", y)
                        } else if let CalcNode::Function(y) = &x.params[1] {
                            format!("w_{}", y.id)
                        } else {
                            panic!("Type not allowed")
                        }
                    )
                    .expect("msg");
                } else if x.params.len() == 1 {
                    write!(
                        sout,
                        "{}({}, {});\n",
                        x.name,
                        format!("w_{}", x.id),
                        if let CalcNode::Text(y) = &x.params[0] {
                            format!("{}", y)
                        } else if let CalcNode::Function(y) = &x.params[0] {
                            format!("w_{}", y.id)
                        } else {
                            panic!("Type not allowed")
                        }
                    )
                    .expect("msg");
                }
            }
            _ => {}
        };
    }
}

pub struct ContextManager {
    contexts: Vec<Context>,
}

impl ContextManager {
    pub fn new() -> ContextManager {
        ContextManager {
            contexts: Vec::new(),
        }
    }

    pub fn push_stack_frame(&mut self, typ: ContextType) {
        let ct = match typ {
            ContextType::Calculate => Context {
                built_in: assemble_map_calc(),
                specific: typ,
                user_def: HashMap::new(),
            },
            ContextType::Verilog => Context {
                built_in: assemble_map_veri(),
                specific: typ,
                user_def: HashMap::new(),
            },
            ContextType::VerilogNand => Context {
                built_in: assemble_map_veri_nand(),
                specific: typ,
                user_def: HashMap::new(),
            },
            ContextType::VerilogNor => Context {
                built_in: assemble_map_veri_nor(),
                specific: typ,
                user_def: HashMap::new(),
            },
        };
        self.contexts.push(ct);
    }

    pub fn pop_stack_frame(&mut self) {
        self.contexts.pop();
    }

    pub fn get_top(&self) -> &Context {
        self.contexts.first().unwrap()
    }

    pub fn get_top_mut(&mut self) -> &mut Context {
        self.contexts.first_mut().unwrap()
    }
}

impl ContextManager {
    pub fn specific(&self) -> ContextType {
        self.get_top().specific
    }

    pub fn get_built_in(&self, k: &'_ str) -> Option<&EvalFunction> {
        self.get_top().built_in.get(k)
    }

    pub fn push_user_def(&mut self, k: String, v: CalcUserFunctionData) {
        self.get_top_mut().user_def.insert(k, v);
    }

    pub fn get_user_def(&self, k: &'_ str) -> Option<&CalcUserFunctionData> {
        self.contexts.iter().rev().find_map(|f| f.user_def.get(k))
    }

    pub fn print_result(&self, ans: &CalcNode) {
        self.get_top().print_result(ans)
    }
}

impl ContextManager {
    pub fn eval(&mut self, node: &CalcNode) -> CalcNode {
        self.eval_internal(node, Arc::new(AtomicUsize::new(0)))
    }

    fn eval_internal(&mut self, node: &CalcNode, counter: Arc<AtomicUsize>) -> CalcNode {
        match node {
            CalcNode::Function(x) => {
                let asd: Vec<CalcNode> = x
                    .params
                    .iter()
                    .map(|y| self.eval_internal(y, counter.clone()))
                    .collect();

                if let Some(t) = self.get_built_in(x.name.as_str()) {
                    return t(asd, counter);
                }

                if let Some(t) = self.get_user_def(x.name.as_str()) {
                    //do a sub and put in
                    // context.push_stack_frame(context.get_top().specific);
                    let mut t = t.clone();
                    t.id = counter.fetch_add(1, Ordering::SeqCst);
                    //sub and eval

                    let ans;
                    if t.params.len() > 0 {
                        self.push_stack_frame(self.specific());
                        for i in 0..t.params.len() {
                            if i < x.params.len() {
                                self.push_user_def(
                                    t.params[i].to_string(),
                                    CalcUserFunctionData {
                                        name: t.params[i].to_string(),
                                        id: counter.fetch_add(1, Ordering::SeqCst),
                                        params: Vec::new(),
                                        eval_tree: Box::new(x.params[i].clone()),
                                    },
                                )
                            }
                        }
                        ans = self.eval_internal(&t.eval_tree, counter);
                        self.pop_stack_frame();
                    } else {
                        ans = self.eval_internal(&t.eval_tree, counter);
                    }

                    return ans;
                } else {
                    return CalcNode::Text(x.name.to_string());
                }
            }
            CalcNode::UserFunction(x) => {
                self.push_user_def(x.name.to_string(), x.clone());
                let fr = x
                    .params
                    .iter()
                    .map(|f| CalcNode::Text(f.to_string()))
                    .collect();
                CalcNode::Function(CalcFunctionData {
                    name: x.name.to_string(),
                    id: x.id,
                    params: fr,
                    operator: None,
                    brackets: false,
                })
            }
            CalcNode::SingleValue(x) => CalcNode::SingleValue(*x),
            CalcNode::Text(x) => CalcNode::Text(x.to_string()),
            _ => panic!("Unexpected"),
        }
    }
}
