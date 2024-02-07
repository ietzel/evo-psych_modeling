import java.util.Scanner;
import processing.core.PApplet;

public class Modeler extends PApplet {
    String[] vars = {"structural auto-evolution", "structural copying of information", "hypercycle-mediating interface", "multi-stage"};
    String[] organization = {"production", "R&D", "administration/management", "purchasing", "revenue"};
    //non-constant (i.e. not too specific), duplication of values between variables (that is, not too vague), saving-deleting cycle (meaning secure carriability in memory), arrayable (in other words, must be plenty of space in program)
    //attention (thoughts), words (actions), habits (character), destiny (environment)
    public void settings() {
	size(500, 500);
    }
    public void draw() {
	ellipse(mouseX, mouseY, 50, 50);
    }
    public void mousePressed() {
	background(64);
    }
    public static void main(String[] args) {
	String[] processingArgs = {"Modeler"};
	Modeler Modeler = new Modeler();
	Scanner s1 = new Scanner(System.in);
	System.out.println("name: ");
	String name = s1.nextLine();
	System.out.println("");
	PApplet.runSketch(processingArgs, Modeler);
    }
}
