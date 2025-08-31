from pprint import pprint as print
from datetime import date
from RustQuant.data import (
    SpotCurve,
    PyCurve,
    CurveType,
    InterpolationMethod,
)
from RustQuant.time import (
    Calendar,
    Market,
)
import RustQuant.data
import RustQuant

RustQuant.data.SpotCurve.new()
RustQuant.__dir__()
RustQuant.data.__dir__()

dates = [
    date(2026, 1, 1),
    date(2027, 1, 2),
    date(2028, 1, 3),
    date(2029, 1, 4),
    date(2030, 1, 5),
]
rates = [
    0.01,
    0.015,
    0.012,
    0.014,
    0.013,
]

crv = PyCurve(dates, rates, CurveType.Spot, InterpolationMethod.Linear)
print(crv.get_rate(date(2026, 6, 1)))


new_dates = [
    date(2026, 6, 1),
    date(2026, 6, 2),
    date(2026, 6, 3),
    date(2026, 6, 4),
    date(2026, 6, 5),
]

curve = SpotCurve(dates, rates)
print(curve.__dir__())

curve.plot()
curve.get_rates(new_dates)

curve.fit()

cal = Calendar(Market.Australia)
print(cal.__dir__())
cal.market()
cal.extra_holidays()
cal.is_business_day(date(2023, 1, 3))
cal.add_holiday(date(2023, 1, 6))
