// TODO: includes

namespace cpp_derive {
{%- for class in classes %}

size_t send(Connector& connector, const {{ class.name }}& item) {
  size_t totalBytes = 0;

  {% for name, field in class.fields -%}
  {%- if field.field_type | striptags == "std::vector" -%}
  // Not necessary when send is properly overloaded for vector
  totalBytes += connector.sendVector(item.{{ name }}, zmq::snd_more);
  {%- else -%}
  totalBytes += connector.send(item.{{ name }}, zmq::snd_more);
  {%- endif %}
  {% endfor %}
  return totalBytes;
}
{%- endfor %}

}
