#include "loginscreen.h"

LoginScreen::LoginScreen(QWidget *parent) : QDialog(parent)
{
    this->parent = parent;
    this->setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Expanding);

    setWindowTitle("Login");
    setFixedSize(350, 200);
    QLabel *usernameLabel = new QLabel("Username:", this);
    QLabel *passwordLabel = new QLabel("Password", this);
    m_usernameEdit = new QLineEdit(this);
    m_passwordEdit = new QLineEdit(this);
    m_passwordEdit->setEchoMode(QLineEdit::Password);

    m_rememberCheckBox = new QCheckBox("Remember me", this);

    m_loginButton = new QPushButton("Login", this);
    m_cancelButton = new QPushButton("Cancel", this);
    m_registerButton = new QPushButton("Register", this);

    m_forgotPasswordLabel = new QLabel("<a href=\"#\">Forgot Password?</a>");
    m_forgotPasswordLabel->setTextFormat(Qt::RichText);
    m_forgotPasswordLabel->setTextInteractionFlags(Qt::TextBrowserInteraction);

    QGridLayout *layout = new QGridLayout(this);
    layout->addWidget(usernameLabel, 0, 0);
    layout->addWidget(m_usernameEdit, 0, 1, 1, 2);

    layout->addWidget(passwordLabel, 1, 0);
    layout->addWidget(m_passwordEdit, 1, 1, 1, 2);

    layout->addWidget(m_rememberCheckBox, 2, 0, 1, 2);
    layout->addWidget(m_forgotPasswordLabel, 2, 2, Qt::AlignRight);

    layout->addWidget(m_loginButton, 3, 1);
    layout->addWidget(m_cancelButton, 3, 2);
    layout->addWidget(m_registerButton, 3, 3);

    connect(m_loginButton, &QPushButton::clicked, this, &LoginScreen::onLoginClicked);
    connect(m_cancelButton, &QPushButton::clicked, this, &QDialog::reject);
    connect(m_forgotPasswordLabel, &QLabel::linkActivated, this, &LoginScreen::onForgotPasswordClicked);
    connect(m_registerButton, &QPushButton::clicked, this, &LoginScreen::onRegisterClicked);

    setTabOrder(m_usernameEdit, m_passwordEdit);
    setTabOrder(m_passwordEdit, m_rememberCheckBox);
    setTabOrder(m_rememberCheckBox, m_loginButton);
    setTabOrder(m_loginButton, m_cancelButton);
    setTabOrder(m_cancelButton, m_registerButton);

    m_usernameEdit->setFocus();
}

void LoginScreen::onLoginClicked()
{
    QString username = m_usernameEdit->text();
    QString password = m_passwordEdit->text();

    if (username.isEmpty() || password.isEmpty()) {
        QMessageBox::warning(this, "Login Error", "Please enter both username and password to login.");
        return;
    }

    if (username == "admin" && password == "password") {
        if (m_rememberCheckBox->isChecked()) {
            saveCredentials(username);
        }
        this->parent->show();
        accept();
    } else {
        QMessageBox::critical(this, "Login Failed", "Invalid username or password");
    }
}
void LoginScreen::onForgotPasswordClicked() {
    QMessageBox::information(this, "Reset Password", "Password reset functionality would be implemented here");
}
void LoginScreen::onRegisterClicked() {
    QMessageBox::information(
        this, "Register Account",
        "Account registration functionality implemented here");
}
void LoginScreen::saveCredentials(const QString &username) {
    qDebug("Remembering login for user: %s", qPrintable(username));
}
