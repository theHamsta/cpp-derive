#include <json.hpp>

namespace cpp_derive {
{%- for class in classes %}

nlohman::json serialize_json(const {{ class.name }}& item) {
  nlohman::json j;
  {% for name, field in class.fields -%}
  j["{{ name }}"] = nlohman::json{item.{{ name }}};
  {% endfor %}
  return j;
}
{%- endfor %}

}
