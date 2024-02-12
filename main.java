import processing.core.PApplet;

public class Modeler extends PApplet {
    String[] vars = {"structural auto-evolution", "structural copying of information", "hypercycle-mediating interface", "multi-stage"};
    String[] organization = {"production", "R&D", "administration/management", "purchasing", "revenue"};
    String[][] business_functions = {
	    {"operations", "environmental sustainability", "supply chain", "product"},
	    {"technology", "information security", "data analytics", "learning"},
	    {"planning", "organizing & staffing", "controlling", "leading"},
	    {"finance/accounting", "responsibility strategy", "HR & A", "UX & customer support"},
	    {"sales/marketing", "communications", "compliance/legal", "party"},
    };
    //non-constant (i.e. not too specific), duplication of values between variables (that is, not too vague), saving-deleting cycle (meaning secure carriability in memory), arrayable (in other words, must be plenty of space in program)
    //attention (thoughts), words (actions), habits (character), destiny (environment)
    public void settings() {
	size(500, 400);
    }
    public void draw() {
	fill(200);
	for(int i = 0; i < organization.length; i++) {
		text(organization[i], i*50, 0);
		for(int j = 0; i < business_functions[i].length; j++) {
			text(business_functions[i][j], i*50, j*40);
		}
	}
    }
    boolean isMouseOver(int x, int y, int w, int h){
	if(mouseX >= x && mouseX <= x + w && mouseY >= y && mouseY <= y + h) {
		return true;
	}
  	return false;
    }

    void mousePressed() {
	if(isMouseOver(width/2,height/2,100,50) == true) {
		println("Mouse pressed button");
    		background = !background;
	}
    }
	    
    public static void main(String[] args) {
	String[] processingArgs = {"Modeler"};
	Modeler Modeler = new Modeler();
	PApplet.runSketch(processingArgs, Modeler);
    }
}
