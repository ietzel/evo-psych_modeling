import { Button, StandardTableView, TabWidget, TextEdit } from "std-widgets.slint";

let vars: [u8; 4] = {"structural auto-evolution", "structural copying of information", "hypercycle-mediating interface", "multi-stage"};
let guide: [u8; 4] = ["attention (thoughts)", "words (actions)", "habits (character)", "destiny (environment)"];
let organization: [u8; 5] = ["production", "R&D", "administration/management", "purchasing", "revenue"];
let business_functions: [[ou8, 4]; 5] = [
        ["operations", "growth", "environmental sustainability", "supply chain"],
	["technology", "information security", "data analysis", "learning"],
	["planning", "organizing & staffing", "controlling", "leading"],
	["finance/accounting", "responsibility strategy", "HR", "UX & customer support"],
	["sales/marketing", "communications", "compliance/legal", "party"]
];
let principles: [u8; 7] = {"honesty/integrity/transparency", "compassion/respect", "responsibility/accountability", "loyalty/trustworthiness", "law-abiding", "fairness", "leadership"};
let policies: [u8; 14] = {"anti-discrimination", "workplace health/safety/security", "employee code of conduct & anti-harassment", "attendance & vacation & time-off", "employee complaint", "work schedule & rest period", "substance abuse", "mobile device management", "compensation and benefits", "travel", "inclement weather", "remote work", "conflict of interest", "acceptable use"};
let conditions: [u8; 5] = {"compensation", "safety", "relationships", "skill discretion", "prospects"};

fn calculate(arr: &mut [i32]) {
	avg = 0;
        for i in range(0, arr.len()) {
	        avg += arr[i];
        }
        return avg/arr.len();
}

slint::slint!{
	export component E_P inherits Window {
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
			Tab {
				title: "Principles/Policies/Conditions";
				Text {
					text: "Principles/Policies/Conditions";
					color: green;
				}
				StandardTableView {
        				columns: [
						{title: "Business Function"},
						{ title: principles[0] },
						{ title: principles[1] },
						{ title: principles[2] },
						{ title: principles[3] },
						{ title: principles[4] },
						{ title: principles[5] },
						{ title: principles[6] },
						{ title: policies[0] },
						{ title: policies[1] },
						{ title: policies[2] },
						{ title: policies[3] },
						{ title: policies[3] },
						{ title: policies[4] },
						{ title: policies[5] },
						{ title: policies[6] },
						{ title: policies[7] },
						{ title: policies[8] },
						{ title: policies[9] },
						{ title: policies[10] },
						{ title: policies[11] },
						{ title: policies[12] },
						{ title: policies[13] },
						{ title: conditions[0] },
						{ title: conditions[1] },
						{ title: conditions[2] },
						{ title: conditions[3] },
						{ title: conditions[4] }
        				];
        				rows: [
            					[
                					{ 
								text:  business_functions[0][0]}, 
								TextEdit {
									
								}
							}
            					],
            					[
                					{ 
								text:  business_functions[0][1]}, 
								TextEdit {
									
								}
							}
            					],
            					[
                					{ 
								text:  business_functions[0][2]}, 
								TextEdit {
									
								}
							}
            					],
						[
                					{ 
								text:  business_functions[0][3]}, 
								TextEdit {
									
								}
							} 
            					],
						[
                						{ 
								text:  business_functions[1][0]}, 
								TextEdit {
									
								}
							}
            					],
            					[
                					{ 
								text:  business_functions[1][1]}, 
								TextEdit {
									
								}
							}
            					],
            					[
                					{ 
								text:  business_functions[1][2]}, 
								TextEdit {
									
								}
							}
            					],
						[
                					{ 
								text:  business_functions[1][3]}, 
								TextEdit {
									
								}
							} 
            					],
						[
                					{ 
								text:  business_functions[2][0]}, 
								TextEdit {
									
								}
							}
            					],
            					[
                					{ 
								text:  business_functions[2][1]}, 
								TextEdit {
									
								}
							}
            					],
            					[
                					{ 
								text:  business_functions[2][2]}, 
								TextEdit {
									
								}
							}
            					],
						[
                					{ 
								text:  business_functions[2][3]}, 
								TextEdit {
									
								}
							} 
            					],
						[
                					{ 
								text:  business_functions[3][0]}, 
								TextEdit {
									
								}
							}
            					],
            					[
                					{ 
								text:  business_functions[3][1]}, 
								TextEdit {
									
								}
							}
            					],
            					[
                					{ 
								text:  business_functions[3][2]}, 
								TextEdit {
									
								}
							}
            					],
						[
                					{ 
								text:  business_functions[3][3]}, 
								TextEdit {
									
								}
							} 
            					],
						[
                					{ 
								text:  business_functions[4][0]}, 
								TextEdit {
									
								}
							}
            					],
            					[
                					{ 
								text:  business_functions[4][1]}, 
								TextEdit {
									
								}
							}
            					],
            					[
                					{ 
								text:  business_functions[4][2]}, 
								TextEdit {
									
								}
							}
            					],
						[
                					{ 
								text:  business_functions[4][3]}, 
								TextEdit {
									
								}
							} 
            					]
					];
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
}

fn main() {
    Modeler::new().unwrap().run().unwrap();
}
