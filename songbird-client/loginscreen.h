#ifndef LOGINSCREEN_H
#define LOGINSCREEN_H

#include <QWidget>
#include <QLabel>
#include <QLineEdit>
#include <QPushButton>
#include <QCheckBox>
#include <QGridLayout>
#include <QMessageBox>
#include "user.h"

class LoginScreen : public QWidget
{
    Q_OBJECT

public:
    explicit LoginScreen(QWidget *parent = nullptr);

signals:
    void loginSuccessful(User& user);
    void registerRequested();

private slots:
    void onLoginClicked();
    void onForgotPasswordClicked();
    void onRegisterClicked();
    void onLoginSuccess(const QJsonObject& responseData, const User& user);
    void onLoginFailure(const QString& errorMessage);

private:
    void saveCredentials(const QString &username);
    void setupConnections();
    QLineEdit *m_usernameEdit;
    QLineEdit *m_passwordEdit;
    QCheckBox *m_rememberCheckBox;
    QPushButton *m_loginButton;
    QPushButton *m_registerButton;
    QLabel *m_forgotPasswordLabel;
};

#endif // LOGINSCREEN_H
