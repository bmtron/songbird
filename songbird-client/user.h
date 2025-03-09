#ifndef USER_H
#define USER_H

#include <QString>

class User
{
public:
    User();
    User(QString& username, QString& password);
    void setPassword(QString& password);
    void setUsername(QString& username);
    void setEmail(QString& email);
    QString password;
    QString username;
    QString email;
};

#endif // USER_H
