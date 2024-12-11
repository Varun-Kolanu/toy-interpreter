const fs = require("fs");
const path = require("path");



function generateAst() {
    const filePath = path.join(__dirname, '../src/ast.rs');
    const base_name = 'Expr'
    const expression_types = {
        "Binary": "left base, operator Token, right base",
        "Grouping": "expression base",
        "Literal": "value LiteralType",
        "Unary": "operator Token, right base"
    }
    let fileContent =
        `use crate::token::{Token,LiteralType};

pub enum ${base_name} {
`;

    let children = [];
    Object.keys(expression_types).forEach(key => {
        let child = `\t${key} {\n`
        expression_types[key].split(",").forEach(field => {
            field = field.trim();
            const k = field.split(" ")[0];
            let v = field.split(" ")[1];
            if (v == "base") v = `Box<${base_name}>`;
            child += `\t\t${k}: ${v},\n`;
        })
        child += "\t},\n";
        children.push(child);
    })
    children.push("}");

    fileContent += children.join("");


    try {
        fs.writeFileSync(filePath, fileContent);
        console.log('File written successfully!');
    } catch (err) {
        console.error('Error writing the file:', err);
    }
}

generateAst();