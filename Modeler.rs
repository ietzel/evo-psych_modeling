slint::slint!{
    export component Modeler inherits Window {
        Text {
            text: "Modeler";
            color: green;
        }
    }
}

let vars: [u8; 4] = {"structural auto-evolution", "structural copying of information", "hypercycle-mediating interface", "multi-stage"};
let organization: [u8; 4] = ["production", "R&D", "administration/management", "purchasing", "revenue"];
let business_functions: [[ou8, 4]; 4] = [
        ["operations", "environmental sustainability", "supply chain", "product"],
	["technology", "information security", "data analytics", "learning"],
	["planning", "organizing & staffing", "controlling", "leading"],
	["finance/accounting", "responsibility strategy", "HR & A", "UX & customer support"],
	["sales/marketing", "communications", "compliance/legal", "party"]
];
let guide: [u8; 4] = ["attention (thoughts)", "words (actions)", "habits (character)", "destiny (environment)"];
const x_lent = 50;
const y_lent = 40;

fn main() {
    Modeler::new().unwrap().run().unwrap();
}
  /*
    fn draw() {
	  for i in &organization {
		  text(organization[i], i*x_lent, 0);
		  for j in &business_functions[i] {
			  text(business_functions[i][j], i*x_lent, j*y_lent);
		  }
	  }
  
    boolean isMouseOver(int x, int y, int w, int h) {
	    if(mouseX >= x && mouseX <= x + w && mouseY >= y && mouseY <= y + h) {
		    return true;
	    }
  	    return false;
      }

      void mousePressed() {
        for(int i = 0; i < organization.length; i++) {
	        for(int j = -1; i < business_functions[i].length; j++) {
	          if(isMouseOver(i*width/10,(j+1)*height/10,(i+1)*width/10,(j+2)*height/10) = true) {
		          if(isMouseOver(width,height,0,height/10) == true) {
		            println(business_functions[i][j]+" business function, maybe in image of "+organization[i]+"business function category.");
		          } else {
		            println(organization[i]+" business function category possibly in good shape.");
		          }
		        }
	        }
        }
     }
}*/
    
