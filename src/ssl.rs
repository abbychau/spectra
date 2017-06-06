use std::collections::HashMap;
use std::str::FromStr;

/// A shader module.
///
/// A shader module is a container that associates some shading code to several identifiers.
struct ShaderModule {
  symbols: HashMap<Identifier, ShadingCode>
}

/// A semigroup version of Vec.
#[derive(Clone, Debug, Eq, PartialEq)]
struct NonEmpty<T>(Vec<T>);

/// Spectra Shading Language AST.
#[derive(Clone, Debug, Eq, PartialEq)]
enum SSL {
  /// An `export list_of_identifiers_` statement.
  Export(NonEmpty<Identifier>),
  /// A `from module use list of identifiers` statement.
  FromUse(Module, NonEmpty<Identifier>),
  /// A `pipeline { list_of_pipeline_attributes }` statement.
  Pipeline(Vec<PipelineAttribute>),
  /// A yield statement, valid in geometry shaders.
  Yield(GeometryYieldExpression),
  /// Some legacy GLSL code.
  GLSL(ShadingCode),
  /// Some more SSL code.
  SSL(Box<SSL>)
}

/// A module.
type Module = String;
/// An identifier.
type Identifier = String;
/// Some opaque shading code.
type ShadingCode = String;
/// An expression.
type Expression = String;

/// Attributes that can be set in a pipeline.
#[derive(Clone, Debug, Eq, PartialEq)]
enum PipelineAttribute {
  /// Maximum vertices that the geometry shader can output.
  GeometryShaderMaxVertices(u32),
  /// Number of times the geometry shader must be invoked.
  GeometryShaderInvokations(u32)
}

/// Expressions that can be yielded in a geometry shader.
#[derive(Clone, Debug, Eq, PartialEq)]
enum GeometryYieldExpression {
  /// Yield a primitive.
  YieldPrimitive,
  /// Yield a primitive’s vertex (fold vertex).
  YieldFoldVertex(Expression)
}

/// Error that can occur when parsing SSL code.
#[derive(Clone, Debug, Eq, PartialEq)]
enum ParseError {
  ExpressionError(String)
}

impl FromStr for PipelineAttribute {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let equal_sign_idx = s.find('=').ok_or(ParseError::ExpressionError("cannot find =".into()))?;
    let (key, value) = s.split_at(equal_sign_idx);

    if value.len() <= 1 {
      return Err(ParseError::ExpressionError("no value".into()));
    }

    match key.trim() {
      "geometry_shader_max_vertices" => {
        let value = (&value[1..]).trim();
        let max_vertices = value.parse().map_err(|_| ParseError::ExpressionError(format!("unable to parse geometry_shader_max_vertices, found {}", value)))?;
        Ok(PipelineAttribute::GeometryShaderMaxVertices(max_vertices))
      },
      "geometry_shader_invokations" => {
        let value = (&value[1..]).trim();
        let invokations = value.parse().map_err(|_| ParseError::ExpressionError(format!("unable to parse geometry_shader_invokation, found {}", value)))?;
        Ok(PipelineAttribute::GeometryShaderInvokations(invokations))
      },
      _ => Err(ParseError::ExpressionError(format!("expected a valid pipeline attribute, found {}", key)))
    }
  }
}

impl FromStr for GeometryYieldExpression {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "yieldprim" => Ok(GeometryYieldExpression::YieldPrimitive),
      _ if s.starts_with("yield ") && s.len() > "yield ".len() => {
        let expr = &s["yield ".len() ..];
        Ok(GeometryYieldExpression::YieldFoldVertex(expr.into()))
      },
      _ => {
        Err(ParseError::ExpressionError(format!("expected yield, found {}", s)))
      }
    }
  }
}
 
// FIXME: move that into a specific module
#[test]
fn parse_pipeline_attribute() {
  let geo_max_vertices  = "geometry_shader_max_vertices = 3";
  let geo_max_vertices1 = "geometry_shader_max_vertices =3";
  let geo_max_vertices2 = "geometry_shader_max_vertices =";
  let geo_max_vertices3 = "geometry_shader_max_vertices = ";
  let geo_invokations  = "geometry_shader_invokations = 1";
  let geo_invokations1 = "geometry_shader_invokations =1";
  let geo_invokations2 = "geometry_shader_invokations =";
  let geo_invokations3 = "geometry_shader_invokations = ";

  assert_eq!(geo_max_vertices.parse::<PipelineAttribute>(), Ok(PipelineAttribute::GeometryShaderMaxVertices(3)));
  assert_eq!(geo_max_vertices1.parse::<PipelineAttribute>(), Ok(PipelineAttribute::GeometryShaderMaxVertices(3)));
  assert!(geo_max_vertices2.parse::<PipelineAttribute>().is_err());
  assert!(geo_max_vertices3.parse::<PipelineAttribute>().is_err());
  assert_eq!(geo_invokations.parse::<PipelineAttribute>(), Ok(PipelineAttribute::GeometryShaderInvokations(1)));
  assert_eq!(geo_invokations1.parse::<PipelineAttribute>(), Ok(PipelineAttribute::GeometryShaderInvokations(1)));
  assert!(geo_invokations2.parse::<PipelineAttribute>().is_err());
  assert!(geo_invokations3.parse::<PipelineAttribute>().is_err());
}

// FIXME: move that into a specific module
#[test]
fn parse_geometry_yield_expression() {
  let yieldprim = "yieldprim";
  let yield_1 = "yield FoldVertex(vertex[i].color)";

  assert_eq!(yieldprim.parse::<GeometryYieldExpression>(), Ok(GeometryYieldExpression::YieldPrimitive));
  assert_eq!(yield_1.parse::<GeometryYieldExpression>(), Ok(GeometryYieldExpression::YieldFoldVertex("FoldVertex(vertex[i].color)".into())));
}