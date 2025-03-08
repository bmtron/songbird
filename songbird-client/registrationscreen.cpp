#include "registrationscreen.h"
#include "constants.h"
#include "networkmanager.h"

RegistrationScreen::RegistrationScreen(QWidget *parent)
    : QWidget(parent)
{
    // Set the window title
    setWindowTitle(Constants::PROJECT_NAME + " - Registration");

    // Create the main layout
    QVBoxLayout *mainLayout = new QVBoxLayout(this);
    mainLayout->setSpacing(20);
    mainLayout->setContentsMargins(40, 40, 40, 40);

    // Create header
    QLabel *headerLabel = new QLabel(Constants::PROJECT_NAME, this);
    QFont headerFont = headerLabel->font();
    headerFont.setPointSize(24);
    headerFont.setBold(true);
    headerLabel->setFont(headerFont);
    headerLabel->setAlignment(Qt::AlignCenter);

    QLabel *subtitleLabel = new QLabel("Create your account", this);
    QFont subtitleFont = subtitleLabel->font();
    subtitleFont.setPointSize(14);
    subtitleLabel->setFont(subtitleFont);
    subtitleLabel->setAlignment(Qt::AlignCenter);

    // Add header to main layout
    mainLayout->addWidget(headerLabel);
    mainLayout->addWidget(subtitleLabel);
    mainLayout->addSpacing(20);

    // Create a centered form container
    QFrame *formContainer = new QFrame(this);
    formContainer->setFrameShape(QFrame::StyledPanel);
    formContainer->setMaximumWidth(500);
    formContainer->setSizePolicy(QSizePolicy::Preferred, QSizePolicy::Maximum);

    QVBoxLayout *containerLayout = new QVBoxLayout(formContainer);
    containerLayout->setSpacing(8);

    // Create form fields
    QLabel *usernameLabel = new QLabel("Username:", this);
    m_usernameEdit = new QLineEdit(this);
    m_usernameEdit->setPlaceholderText("Choose a username");

    QLabel *emailLabel = new QLabel("Email:", this);
    m_emailEdit = new QLineEdit(this);
    m_emailEdit->setPlaceholderText("Enter your email address");

    QLabel *passwordLabel = new QLabel("Password:", this);
    m_passwordEdit = new QLineEdit(this);
    m_passwordEdit->setEchoMode(QLineEdit::Password);
    m_passwordEdit->setPlaceholderText("Choose a secure password");

    QLabel *passwordConfirmLabel = new QLabel("Confirm Password:", this);
    m_passwordConfirmEdit = new QLineEdit(this);
    m_passwordConfirmEdit->setEchoMode(QLineEdit::Password);
    m_passwordConfirmEdit->setPlaceholderText("Re-enter your password");

    // Add form fields to container layout
    containerLayout->addWidget(usernameLabel);
    containerLayout->addWidget(m_usernameEdit);
    containerLayout->addWidget(emailLabel);
    containerLayout->addWidget(m_emailEdit);
    containerLayout->addWidget(passwordLabel);
    containerLayout->addWidget(m_passwordEdit);
    containerLayout->addWidget(passwordConfirmLabel);
    containerLayout->addWidget(m_passwordConfirmEdit);

    // Create button layout
    QHBoxLayout *buttonLayout = new QHBoxLayout();
    buttonLayout->setSpacing(15);

    m_cancelButton = new QPushButton("Cancel", this);
    m_registerButton = new QPushButton("Register", this);
    m_registerButton->setDefault(true);
    m_registerButton->setAutoDefault(true);

    // Style the register button to stand out
    m_registerButton->setStyleSheet("background-color: #0a36d1; color: white;");

    // Add buttons to button layout
    buttonLayout->addStretch();
    buttonLayout->addWidget(m_cancelButton);
    buttonLayout->addWidget(m_registerButton);

    // Add button layout to container
    containerLayout->addSpacing(10);
    containerLayout->addLayout(buttonLayout);

    // Add container to main layout with horizontal centering
    QHBoxLayout *centeringLayout = new QHBoxLayout();
    centeringLayout->addStretch();
    centeringLayout->addWidget(formContainer);
    centeringLayout->addStretch();
    mainLayout->addLayout(centeringLayout);
    mainLayout->addStretch();

    // Add a footer with additional information
    QLabel *privacyNote = new QLabel("By registering, you agree to our <a href=\"#\">Terms of Service</a> and <a href=\"#\">Privacy Policy</a>.", this);
    privacyNote->setTextFormat(Qt::RichText);
    privacyNote->setTextInteractionFlags(Qt::TextBrowserInteraction);
    privacyNote->setAlignment(Qt::AlignCenter);
    privacyNote->setOpenExternalLinks(false);

    QLabel *loginPrompt = new QLabel("Already have an account? <a href=\"#\">Log in</a>", this);
    loginPrompt->setTextFormat(Qt::RichText);
    loginPrompt->setTextInteractionFlags(Qt::TextBrowserInteraction);
    loginPrompt->setAlignment(Qt::AlignCenter);

    mainLayout->addWidget(privacyNote);
    mainLayout->addWidget(loginPrompt);

    // Connect signals and slots
    connect(m_registerButton, &QPushButton::clicked, this, &RegistrationScreen::onRegisterClicked);
    connect(m_cancelButton, &QPushButton::clicked, this, &RegistrationScreen::onCancelClicked);
    connect(loginPrompt, &QLabel::linkActivated, this, &RegistrationScreen::onCancelClicked);

    // Set tab order
    setTabOrder(m_usernameEdit, m_emailEdit);
    setTabOrder(m_emailEdit, m_passwordEdit);
    setTabOrder(m_passwordEdit, m_passwordConfirmEdit);
    setTabOrder(m_passwordConfirmEdit, m_registerButton);
    setTabOrder(m_registerButton, m_cancelButton);

    // Set initial focus
    m_usernameEdit->setFocus();
}

bool RegistrationScreen::validateForm()
{
    // Check if username is empty
    if (m_usernameEdit->text().isEmpty()) {
        QMessageBox::warning(this, "Registration Error", "Please enter a username.");
        m_usernameEdit->setFocus();
        return false;
    }

    // Check if email is empty or invalid
    QString email = m_emailEdit->text();
    if (email.isEmpty() || !email.contains('@') || !email.contains('.')) {
        QMessageBox::warning(this, "Registration Error", "Please enter a valid email address.");
        m_emailEdit->setFocus();
        return false;
    }

    // Check if password is empty or too short
    QString password = m_passwordEdit->text();
    if (password.isEmpty() || password.length() < Constants::PASSWORD_MIN_LEN) {
        QMessageBox::warning(this, "Registration Error", "Password must be at least 8 characters long.");
        m_passwordEdit->setFocus();
        return false;
    }

    // Check if passwords match
    if (password != m_passwordConfirmEdit->text()) {
        QMessageBox::warning(this, "Registration Error", "Passwords do not match.");
        m_passwordConfirmEdit->setFocus();
        return false;
    }

    return true;
}

void RegistrationScreen::onRegisterClicked()
{
    if (!validateForm()) {
        return;
    }

    // Create a user object
    User newUser;
    // Set user properties (assuming User class has these methods)
    newUser.setUsername(m_usernameEdit->text());
    newUser.setEmail(m_emailEdit->text());
    newUser.setPassword(m_passwordEdit->text());

    // Submit user
    submitNewUser(newUser);

    // Show success message
    QMessageBox::information(this, "Registration Successful",
                             "Your account has been created successfully!");

    // Emit signal that registration is complete
    emit registrationCompleted(newUser);
}

void RegistrationScreen::onCancelClicked()
{
    // Emit signal that registration was cancelled
    emit registrationCancelled();
}

void RegistrationScreen::submitNewUser(User &user)
{
    // Placeholder for API call or database storage
    // In a real implementation, this would connect to your backend
    qDebug("Creating new user: %s", qPrintable(m_usernameEdit->text()));

    NetworkManager &netMan = NetworkManager::instance();
    netMan.registerUser(user);
    // Here you would typically:
    // 1. Hash the password
    // 2. Make an API call to your Rust backend
    // 3. Handle the response
}
