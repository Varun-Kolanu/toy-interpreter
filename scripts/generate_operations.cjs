const fs = require("fs");
const path = require("path");

function generateOperation() {
    const args = process.argv;
    if (args.length <= 4) {
        console.log("Usage: node ./generate_operations.cjs <operationClassName> <genericType> <fileName>");
    }
    const operation = args[2];
    const genericType = args[3];
    const fileName = args[4];
    const filePath = path.join(__dirname, `../src/${fileName}`);
    const base_name = "Expr";

    const expression_types = {
        "Binary": "left base, operator Token, right base",
        "Grouping": "expression base",
        "Literal": "value LiteralType",
        "Unary": "operator Token, right base"
    }

    let fileContent =
        `use crate::ast::{Visitor, Expr};
use crate::token::{Token,LiteralType};

struct ${operation} {}

impl Visitor<${genericType}> for ${operation} {
`;

    let children = [];

    // Visitor
    Object.keys(expression_types).forEach(key => {
        let child = `\tfn visit_${key.toLowerCase()}_${base_name.toLowerCase()}(&mut self,`
        expression_types[key].split(",").forEach(field => {
            field = field.trim();
            const k = field.split(" ")[0];
            let v = field.split(" ")[1];
            if (v == "base") v = `${base_name}`;
            child += ` ${k}: &${v},`;
        })
        child += `) -> ${genericType} {}\n\n`;
        children.push(child);
    })

    children.push("}\n\n");

    fileContent += children.join("");


    try {
        fs.writeFileSync(filePath, fileContent);
        console.log('File written successfully!');
    } catch (err) {
        console.error('Error writing the file:', err);
    }
}

generateOperation();