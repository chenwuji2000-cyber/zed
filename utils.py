def calculate_grade(score):
    if score >= 90:
        return "A"
    elif score >= 80:
        return "B"
    elif score >= 70:
        return "C"
    elif score >= 60:
        return "D"
    else:
        return "F"


def format_report(name, score):
    grade = calculate_grade(score)
    return f"{name}: {score} ({grade})"


def print_summary(students):
    for name, score in students.items():
        print(format_report(name, score))
