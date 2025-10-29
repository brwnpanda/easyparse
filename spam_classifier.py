#!/usr/bin/env python3
"""
Spam Email Classifier using NLP and Classical ML
-------------------------------------------------
This module implements a spam email classifier that compares 
Naive Bayes and Logistic Regression models using TF-IDF vectorization.
"""

import re
from sklearn.model_selection import train_test_split
from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.naive_bayes import MultinomialNB
from sklearn.linear_model import LogisticRegression
from sklearn.metrics import accuracy_score, classification_report, confusion_matrix
import warnings
warnings.filterwarnings('ignore')


def preprocess_text(text):
    """
    Preprocess text data with basic NLP techniques.
    
    Args:
        text (str): Input text to preprocess
        
    Returns:
        str: Preprocessed text
    """
    # Convert to lowercase
    text = text.lower()
    
    # Remove special characters and digits
    text = re.sub(r'[^a-zA-Z\s]', '', text)
    
    # Remove extra whitespace
    text = re.sub(r'\s+', ' ', text).strip()
    
    return text


def create_sample_dataset():
    """
    Create a sample dataset of spam and ham (non-spam) emails.
    
    Returns:
        tuple: (emails, labels) where emails is a list of email texts
               and labels is a list of corresponding labels (0=ham, 1=spam)
    """
    # Sample spam emails
    spam_emails = [
        "Congratulations! You've won a free iPhone. Click here to claim now!",
        "Get rich quick! Invest in our amazing opportunity. Limited time offer!",
        "URGENT: Your account will be closed. Verify your information immediately.",
        "You have won $1,000,000 in the lottery. Send your bank details to claim.",
        "Free money! No strings attached. Act now before it's too late!",
        "Viagra at lowest prices. Order now and get 50% discount!",
        "Work from home and earn $5000 per week. No experience needed!",
        "CLICK HERE for free gift cards and amazing prizes!",
        "Your computer is infected! Download our antivirus software now.",
        "Lose 20 pounds in 2 weeks with this miracle pill!",
        "You've been selected for a special credit card offer. Apply today!",
        "Get a university degree in 2 weeks. No exams required!",
        "Free vacation to the Bahamas! Just pay processing fee.",
        "Increase your income by 500% with this simple trick!",
        "Urgent: Update your payment information to avoid suspension.",
        "You are the lucky winner of our cash prize draw!",
        "Meet singles in your area tonight! Sign up free!",
        "Your refund of $500 is ready. Click to receive payment.",
        "Earn money online from home. Start making thousands today!",
        "Exclusive offer: Buy prescription drugs without prescription.",
    ]
    
    # Sample legitimate emails (ham)
    ham_emails = [
        "Hi, let's meet for coffee tomorrow at 3 PM to discuss the project.",
        "Your order has been shipped and will arrive in 3-5 business days.",
        "Reminder: Team meeting scheduled for Monday at 10 AM in conference room.",
        "Thank you for your purchase. Your receipt is attached to this email.",
        "The quarterly report is ready for review. Please find it attached.",
        "Your subscription renewal is due next month. No action needed now.",
        "Welcome to our newsletter! Here are this week's top articles.",
        "Your appointment is confirmed for Tuesday at 2 PM with Dr. Smith.",
        "Project deadline has been extended to next Friday. Please adjust accordingly.",
        "Here are the meeting notes from yesterday's discussion.",
        "Your password reset request was successful. You can now log in.",
        "Reminder: Please submit your timesheet by end of day Friday.",
        "The event you registered for will take place on Saturday at the convention center.",
        "Your flight booking confirmation for next month is attached.",
        "Thank you for attending our webinar. The recording is now available.",
        "Your monthly statement is now available in your online account.",
        "The software update has been completed successfully on your system.",
        "Please review the attached document and provide your feedback.",
        "Your library books are due for return next week.",
        "The new company policy document has been uploaded to the portal.",
    ]
    
    # Combine emails and create labels
    emails = spam_emails + ham_emails
    labels = [1] * len(spam_emails) + [0] * len(ham_emails)  # 1 = spam, 0 = ham
    
    return emails, labels


def train_and_evaluate_models(X_train, X_test, y_train, y_test):
    """
    Train and evaluate both Naive Bayes and Logistic Regression models.
    
    Args:
        X_train: Training features
        X_test: Test features
        y_train: Training labels
        y_test: Test labels
        
    Returns:
        tuple: (nb_model, lr_model) - trained models
    """
    print("\n" + "="*70)
    print("TRAINING AND EVALUATING MODELS")
    print("="*70)
    
    # Train Naive Bayes
    print("\n1. Naive Bayes Classifier")
    print("-" * 70)
    nb_model = MultinomialNB()
    nb_model.fit(X_train, y_train)
    nb_predictions = nb_model.predict(X_test)
    nb_accuracy = accuracy_score(y_test, nb_predictions)
    
    print(f"Accuracy: {nb_accuracy:.2%}")
    print("\nClassification Report:")
    print(classification_report(y_test, nb_predictions, target_names=['Ham', 'Spam']))
    print("\nConfusion Matrix:")
    print(confusion_matrix(y_test, nb_predictions))
    
    # Train Logistic Regression
    print("\n2. Logistic Regression Classifier")
    print("-" * 70)
    lr_model = LogisticRegression(max_iter=1000, random_state=42)
    lr_model.fit(X_train, y_train)
    lr_predictions = lr_model.predict(X_test)
    lr_accuracy = accuracy_score(y_test, lr_predictions)
    
    print(f"Accuracy: {lr_accuracy:.2%}")
    print("\nClassification Report:")
    print(classification_report(y_test, lr_predictions, target_names=['Ham', 'Spam']))
    print("\nConfusion Matrix:")
    print(confusion_matrix(y_test, lr_predictions))
    
    # Comparison
    print("\n" + "="*70)
    print("MODEL COMPARISON")
    print("="*70)
    print(f"Naive Bayes Accuracy:       {nb_accuracy:.2%}")
    print(f"Logistic Regression Accuracy: {lr_accuracy:.2%}")
    
    if nb_accuracy > lr_accuracy:
        print("\n✓ Naive Bayes performs better on this dataset")
    elif lr_accuracy > nb_accuracy:
        print("\n✓ Logistic Regression performs better on this dataset")
    else:
        print("\n✓ Both models perform equally well on this dataset")
    
    return nb_model, lr_model


def predict_email(email_text, vectorizer, model, model_name="Model"):
    """
    Predict whether an email is spam or ham.
    
    Args:
        email_text (str): Email text to classify
        vectorizer: Fitted TF-IDF vectorizer
        model: Trained classification model
        model_name (str): Name of the model for display
        
    Returns:
        tuple: (prediction, probability)
    """
    processed_text = preprocess_text(email_text)
    features = vectorizer.transform([processed_text])
    prediction = model.predict(features)[0]
    probability = model.predict_proba(features)[0]
    
    label = "SPAM" if prediction == 1 else "HAM"
    confidence = probability[prediction] * 100
    
    return label, confidence


def main():
    """
    Main function to demonstrate the spam email classifier.
    """
    print("="*70)
    print("SPAM EMAIL CLASSIFIER - NLP & CLASSICAL ML")
    print("="*70)
    print("\nUsing TF-IDF Vectorization and Classical ML Models")
    print("Comparing Naive Bayes vs Logistic Regression\n")
    
    # Create dataset
    print("Loading dataset...")
    emails, labels = create_sample_dataset()
    print(f"Total emails: {len(emails)}")
    print(f"Spam emails: {sum(labels)}")
    print(f"Ham emails: {len(labels) - sum(labels)}")
    
    # Preprocess all emails
    print("\nPreprocessing emails...")
    processed_emails = [preprocess_text(email) for email in emails]
    
    # Split data
    print("Splitting data into train/test sets (80/20)...")
    X_train_text, X_test_text, y_train, y_test = train_test_split(
        processed_emails, labels, test_size=0.2, random_state=42, stratify=labels
    )
    
    # TF-IDF Vectorization
    print("Applying TF-IDF vectorization...")
    vectorizer = TfidfVectorizer(max_features=1000, stop_words='english')
    X_train = vectorizer.fit_transform(X_train_text)
    X_test = vectorizer.transform(X_test_text)
    
    print(f"Feature matrix shape: {X_train.shape}")
    print(f"Number of features: {X_train.shape[1]}")
    
    # Train and evaluate models
    nb_model, lr_model = train_and_evaluate_models(X_train, X_test, y_train, y_test)
    
    # Test with new examples
    print("\n" + "="*70)
    print("TESTING WITH NEW EXAMPLES")
    print("="*70)
    
    test_emails = [
        "Congratulations! You won a million dollars! Claim now!",
        "Hi team, the meeting is scheduled for tomorrow at 10 AM.",
        "URGENT: Your account needs verification immediately!",
        "Your package has been delivered. Thank you for your order.",
    ]
    
    for i, email in enumerate(test_emails, 1):
        print(f"\nTest Email #{i}:")
        print(f"Text: {email}")
        
        # Predict with Naive Bayes
        nb_label, nb_conf = predict_email(email, vectorizer, nb_model, "Naive Bayes")
        print(f"  Naive Bayes:       {nb_label} (confidence: {nb_conf:.1f}%)")
        
        # Predict with Logistic Regression
        lr_label, lr_conf = predict_email(email, vectorizer, lr_model, "Logistic Regression")
        print(f"  Logistic Regression: {lr_label} (confidence: {lr_conf:.1f}%)")
    
    print("\n" + "="*70)
    print("SPAM CLASSIFIER DEMONSTRATION COMPLETE")
    print("="*70)


if __name__ == "__main__":
    main()
