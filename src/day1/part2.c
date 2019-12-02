#include <stdio.h>
#include <stdlib.h>

int calcFuelReq(int mass)
{
	int curMass = 0;
	int retMass = 0;
	curMass = mass/3;
	curMass -= 2;

	if (curMass >= 0){
		curMass += calcFuelReq(curMass);
		retMass = curMass;
	}

	return retMass;
}


int main(int argc, char *argv[])
{
	FILE *f;
	int bufSize = 1024;
	char buf[bufSize];
	int totalFuel = 0;

	f = fopen("input.txt", "r");
	while(1){
		char *ret = fgets(buf, bufSize, f);
		if (ret == 0x00){
			break;
		}

		int moduleMass = atoi(buf);
		totalFuel += calcFuelReq(moduleMass);
	}
	fclose(f);

	printf("Total Fuel: %d\n", totalFuel);

	return 0; 
}
