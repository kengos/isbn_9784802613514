height_cm = float(input("身長(cm)は?"))
weight = float(input("体重(kg)は?"))

height = height_cm / 100.0
bmi = weight / (height ** 2)

bmi_list = [
    {"min": 0, "max":  18.5, "label": "低体重"},
    {"min": 18.5, "max":  25, "label": "普通体重"},
    {"min": 25, "max":  30, "label": "肥満1度"},
    {"min": 30, "max":  35, "label": "肥満2度"},
    {"min": 35, "max":  40, "label": "肥満3度"},
    {"min": 40, "max":  99, "label": "肥満4度"},
]

result = "不明"
for s in bmi_list:
    if s["min"] <= bmi < s["max"]:
        result = s["label"]
        break

print("BMI={:.1f},判定={}".format(bmi, result))
