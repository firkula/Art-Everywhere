import java.util.Scanner;

public class ArtEverywhere {
	public static void main(String[] args) {
		Scanner input = new Scanner(System.in);
		int decision;
		//prints the welcome statement
		System.out.println("Welcome to Art Everywhere!\n");
		// while loop goes until user decides to quit
		while (true) {
			//prints options user can choose from
			System.out.println("Choose one of the following options: \n"
					+ "1. Paint a Picture \n"
					+ "2. Sculpt a Statue \n"
					+ "3. Create a Sculpture \n"
					+ "4. Design a Mosaic \n"
					+ "5. Quit \n");
			//gets the user's decision
			decision = input.nextInt();
			
			//if statement takes the user's decision and prints the
			//appropriate response
			if (decision == 1) {
				System.out.println("You can look at different painting techniques"
						+ " and materials online to get started!\n");
			}
			else if (decision == 2) {
				System.out.println("Visit a local art store to find the materials"
						+ " you need to get started!\n");
			}
			else if (decision == 3) {
				System.out.println("Look online to find out the different types" 
						+ " of sculptures you can make!\n");
			}
			else if (decision == 4) {
				System.out.println("Look at different tutorials to learn" 
						+ " how to make a mosaic!\n");
			}
			else if (decision == 5) {
				break;
			}
			else {
				System.out.println("Invalid input. Please choose from the"
						+ " presented options!\n");
			}
		}
		//closes scanner
		input.close();
		System.out.println("Thank you for using Art Everywhere! Have a great day!");
	}
}