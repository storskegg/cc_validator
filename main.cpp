#include <iostream>
#include "validator.h"

using namespace std;
using namespace validator;


int main() {
    const Validator v{};

    cout << "Enter a string to be validated: " << endl;

    cin.getline(v.getData(), 32);

    cout << "Length of the string: " << strlen(v.getData()) << endl;

    cout << "----[ Print ]-----------------------" << endl;
    v.print();

    cout << "----[ Vet Input ]--------------------" << endl;
    if (v.vetInput()) {
        cout << "Input is valid." << endl;
    } else {
        cout << "Input is not valid." << endl;
    }

    return 0;
}
