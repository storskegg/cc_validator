#include <iostream>
#include "validator.h"

using namespace std;
using namespace validator;

 /**
 * Main thread.
 *
 * The user is first prompted to enter a number string, which is then printed
 * to console for verification (mostly to show whether the input was truncated).
 * The input is then validated using the Luhn algorithm, and the results
 * displayed.
 *
 * @return the Result from the validation process. (0 success, 1 invalid, etc.))
 */
int main() {
    const auto v = new Validator();

    cout << "Enter a string to be validated: " << endl;

    cin.getline(v->getData(), MAX_DATA);

    v->printDigits();

    const Result v_result = v->isValid();
    switch (v_result) {
        case VALID:
            cout << "Input is valid." << endl;
            break;
        case INVALID:
            cout << "Input is not valid." << endl;
            break;
        case ERR_BAD_INPUT:
            cout << "Input contains invalid characters." << endl;
            break;
        // Add more cases as needed
    }

    return v_result;
}
