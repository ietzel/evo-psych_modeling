using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace main
  static class Program {
    /*String[] vars = {"structural auto-evolution", "structural copying of information", "hypercycle-mediating interface", "multi-stage"};
    String[] organization = {"production", "R&D", "administration/management", "purchasing", "revenue"};
    String[][] business_functions = {
	    {"operations", "environmental sustainability", "supply chain", "product"},
	    {"technology", "information security", "data analytics", "learning"},
	    {"planning", "organizing & staffing", "controlling", "leading"},
	    {"finance/accounting", "responsibility strategy", "HR & A", "UX & customer support"},
	    {"sales/marketing", "communications", "compliance/legal", "party"}
    };
    String[] guide = {"attention (thoughts)", "words (actions)", "habits (character)", "destiny (environment)"};
    const x_lent = 50;
    const y_lent = 40;
    public void draw() {
	  for(int i = 0; i < organization.length; i++) {
		  text(organization[i], i*x_lent, 0);
		  for(int j = 0; i < business_functions[i].length; j++) {
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
     }*/
    [STAThread]
    public Main() {
      Application.enableVisualStyles();
      Application.setCompatibleTextRenderingDefault(false);
      Application.Run(new main());
    }
    
