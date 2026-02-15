from dataclasses import dataclass
from typing import Optional


@dataclass
class GradeThreshold:
    min_score: int
    grade: str


GRADE_THRESHOLDS = [
    GradeThreshold(90, "A"),
    GradeThreshold(80, "B"),
    GradeThreshold(70, "C"),
    GradeThreshold(60, "D"),
]


def calculate_grade(score: int) -> str:
    for threshold in GRADE_THRESHOLDS:
        if score >= threshold.min_score:
            return threshold.grade
    return "F"


def format_report(name: str, score: int, include_grade: bool = True) -> str:
    grade = calculate_grade(score)
    if include_grade:
        return f"{name}: {score} ({grade})"
    return f"{name}: {score}"


def print_summary(students: dict[str, int], title: Optional[str] = None) -> None:
    if title:
        print(f"=== {title} ===")
        print()

    for name, score in sorted(students.items(), key=lambda x: x[1], reverse=True):
        print(format_report(name, score))

    scores = list(students.values())
    avg = sum(scores) / len(scores) if scores else 0
    print(f"\nAverage: {avg:.1f}")


def export_csv(students: dict[str, int], filename: str = "report.csv") -> None:
    with open(filename, "w") as f:
        f.write("Name,Score,Grade\n")
        for name, score in sorted(students.items()):
            grade = calculate_grade(score)
            f.write(f"{name},{score},{grade}\n")
