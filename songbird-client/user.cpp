#include "user.h"

User::User() {}

User::User(QString& username, QString& password) {
    this->username = username;
    this->password = password;
}

void User::setUsername(QString& username) {
    this->username = username;
}
void User::setEmail(QString& email) {
    this->email = email;
}
void User::setPassword(QString& password) {
    this->password = password;
}
