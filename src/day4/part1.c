#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int checkPass(char *pass)
{
	// prep some vars
	int passLen = sizeof(pass)/sizeof(char)-1; // get rid of the newline count
	int adjacent = 0;
	int increase = 0;
	for (int i = 0; i < passLen-1; i++){
		// type each char
		char first = pass[i];
		char second = pass[i+1];

		// check if adjacent digits are the same
		if (first == second){
			adjacent++;
		}

		// check if digits increase
		if (first <= second){
			increase++;
		}
	}			

	// ret success if both flags are set
	int retval = 0;
	if (adjacent > 0 && increase == 5){
		retval = 1;
	}
	return retval;
}

int main(int argc, char *argv[])
{
	int passSize = 7;
	int puzzleStart = 134792;
	int puzzleEnd = 675810;

	int numValidPasswds = 0;

	for (int i = puzzleStart; i < puzzleEnd+1; i++){
		// get the input into string form
		char* curPass;
		curPass = malloc(passSize * sizeof(char));
		snprintf(curPass, passSize, "%d", i);

		// perform check
		if (checkPass(curPass) == 1){
			numValidPasswds++;
		}
	}

	printf("Num Valid Passwords: %d\n", numValidPasswds);

	return 0;
}