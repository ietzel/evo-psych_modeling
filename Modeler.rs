import { Button, StandardTableView } from "std-widgets.slint";

/*let guide: [u8; 4] = ["attention (thoughts)", "words (actions)", "habits (character)", "destiny (environment)"];
let vars: [u8; 4] = {"structural auto-evolution", "structural copying of information", "hypercycle-mediating interface", "multi-stage"};*/
let organization: [u8; 5] = ["production", "R&D", "administration/management", "purchasing", "revenue"];
let business_functions: [[ou8, 4]; 5] = [
        ["operations", "environmental sustainability", "supply chain", "product"],
	["technology", "information security", "data analytics", "learning"],
	["planning", "organizing & staffing", "controlling", "leading"],
	["finance/accounting", "responsibility strategy", "HR & A", "UX & customer support"],
	["sales/marketing", "communications", "compliance/legal", "party"]
];

slint::slint!{
    export component Modeler inherits Window {
        Text {
            text: "Modeler";
            color: green;
        }
	Button { text: organization[0]; }
	Button { text: organization[1]; }
	Button { text: organization[2]; }
	Button { text: organization[3]; }
	Button { text: organization[4]; }
	StandardTableView {
        	columns: [
			{ title: organization[0] },
            		{ title: organization[1] },
	    		{ title: organization[2] },
            		{ title: organization[3] },
            		{ title: organization[4] },
        	];
        	rows: [
            		[
                		{ text:  business_functions[0][0]}, { text: business_functions[0][1] }, { text:  business_functions[0][2]}, { text: business_functions[0][3] },
            		],
            		[
                		{ text:  business_functions[1][0]}, { text: business_functions[1][1] }, { text:  business_functions[1][2]}, { text: business_functions[1][3] },
            		],
            		[
                		{ text:  business_functions[2][0]}, { text: business_functions[2][1] }, { text:  business_functions[2][2]}, { text: business_functions[2][3] },
            		],
			[
                		{ text:  business_functions[3][0]}, { text: business_functions[3][1] }, { text:  business_functions[3][2]}, { text: business_functions[3][3] },
            		],
            		[
                		{ text:  business_functions[4][0]}, { text: business_functions[4][1] }, { text:  business_functions[4][2]}, { text: business_functions[4][3] },
            		]
        	];
	}
}

fn main() {
    Modeler::new().unwrap().run().unwrap();
}
