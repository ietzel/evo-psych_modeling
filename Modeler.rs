import { Button, StandardTableView, TabWidget } from "std-widgets.slint";

let vars: [u8; 4] = {"structural auto-evolution", "structural copying of information", "hypercycle-mediating interface", "multi-stage"};
let guide: [u8; 4] = ["attention (thoughts)", "words (actions)", "habits (character)", "destiny (environment)"];
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
		width: 1024px;
    		height: 1024px;
		TabWidget {
			Tab {
				title: "Organization(s)";
        			Text {
            				text: "Modeler";
            				color: green;
        			}
				Button { text: "Concern beyond People"; }
				Button { text: "Concern to People"; }    
				Button { text: organization[0]; }
				Button { text: organization[1]; }
				Button { text: organization[2]; }
				Button { text: organization[3]; }
				Button { text: organization[4]; }
				StandardTableView {
        				columns: [
						{ title: '"Level of Concern beyond People"'}
						{ title: '"Level of Concern to People"'}
						{ title: organization[0] },
        			    		{ title: organization[1] },
	    					{ title: organization[2] },
            					{ title: organization[3] },
            					{ title: organization[4] },
        					];
        					rows: [
							[
                					{ text:  vars[0]}, { text:  vars[1] }, { text:  vars[2]}, { text: vars[3] },
            					],
						[
        	        				{ text:  guide[3]}, { text:  guide[2] }, { text:  guide[1]}, { text: guide[0] },
            					],
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
		}
		Tab {
			title: "Graph";
			Text {
				text: "Graph";
				color: green;
			}
		}
	}
}

fn main() {
    Modeler::new().unwrap().run().unwrap();
}
