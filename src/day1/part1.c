#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[])
{
	FILE *f;
	int bufSize = 1024;
	char buf[bufSize];
	int total = 0;

	f = fopen("input.txt", "r");
	while(1){
		char *ret = fgets(buf, bufSize, f);
		if (ret == 0x00){
			break;
		}

		int curVal = atoi(buf);
		curVal = curVal/3;
		curVal -= 2;
		total += curVal;
	}
	fclose(f);

	printf("Total: %d\n", total);

	return 0; 
}
