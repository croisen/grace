// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint, type=warning, deprecated_member_use, deprecated_member_use_from_same_package
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'rust.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$RustAst {





@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is RustAst);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
    return 'RustAst()';
}


}

/// @nodoc
class $RustAstCopyWith<$Res>  {
$RustAstCopyWith(RustAst _, $Res Function(RustAst) __);
}


/// Adds pattern-matching-related methods to [RustAst].
extension RustAstPatterns on RustAst {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( RustAst_Import value)?  import_,TResult Function( RustAst_VariableDecl value)?  variableDecl,TResult Function( RustAst_FunctionDecl value)?  functionDecl,TResult Function( RustAst_FunctionImpl value)?  functionImpl,TResult Function( RustAst_FunctionCall value)?  functionCall,TResult Function( RustAst_StructDecl value)?  structDecl,TResult Function( RustAst_RawStruct value)?  rawStruct,TResult Function( RustAst_EnumDecl value)?  enumDecl,TResult Function( RustAst_RawEnum value)?  rawEnum,TResult Function( RustAst_ZeImpl value)?  zeImpl,TResult Function( RustAst_IfConditional value)?  ifConditional,TResult Function( RustAst_IfLet value)?  ifLet,TResult Function( RustAst_ForLoop value)?  forLoop,TResult Function( RustAst_WhileLoop value)?  whileLoop,TResult Function( RustAst_WhileLet value)?  whileLet,TResult Function( RustAst_TraitDecl value)?  traitDecl,TResult Function( RustAst_Unknown value)?  unknown,required TResult orElse(),}){
final _that = this;
switch (_that) {
case RustAst_Import() when import_ != null:
return import_(_that);case RustAst_VariableDecl() when variableDecl != null:
return variableDecl(_that);case RustAst_FunctionDecl() when functionDecl != null:
return functionDecl(_that);case RustAst_FunctionImpl() when functionImpl != null:
return functionImpl(_that);case RustAst_FunctionCall() when functionCall != null:
return functionCall(_that);case RustAst_StructDecl() when structDecl != null:
return structDecl(_that);case RustAst_RawStruct() when rawStruct != null:
return rawStruct(_that);case RustAst_EnumDecl() when enumDecl != null:
return enumDecl(_that);case RustAst_RawEnum() when rawEnum != null:
return rawEnum(_that);case RustAst_ZeImpl() when zeImpl != null:
return zeImpl(_that);case RustAst_IfConditional() when ifConditional != null:
return ifConditional(_that);case RustAst_IfLet() when ifLet != null:
return ifLet(_that);case RustAst_ForLoop() when forLoop != null:
return forLoop(_that);case RustAst_WhileLoop() when whileLoop != null:
return whileLoop(_that);case RustAst_WhileLet() when whileLet != null:
return whileLet(_that);case RustAst_TraitDecl() when traitDecl != null:
return traitDecl(_that);case RustAst_Unknown() when unknown != null:
return unknown(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( RustAst_Import value)  import_,required TResult Function( RustAst_VariableDecl value)  variableDecl,required TResult Function( RustAst_FunctionDecl value)  functionDecl,required TResult Function( RustAst_FunctionImpl value)  functionImpl,required TResult Function( RustAst_FunctionCall value)  functionCall,required TResult Function( RustAst_StructDecl value)  structDecl,required TResult Function( RustAst_RawStruct value)  rawStruct,required TResult Function( RustAst_EnumDecl value)  enumDecl,required TResult Function( RustAst_RawEnum value)  rawEnum,required TResult Function( RustAst_ZeImpl value)  zeImpl,required TResult Function( RustAst_IfConditional value)  ifConditional,required TResult Function( RustAst_IfLet value)  ifLet,required TResult Function( RustAst_ForLoop value)  forLoop,required TResult Function( RustAst_WhileLoop value)  whileLoop,required TResult Function( RustAst_WhileLet value)  whileLet,required TResult Function( RustAst_TraitDecl value)  traitDecl,required TResult Function( RustAst_Unknown value)  unknown,}){
final _that = this;
switch (_that) {
case RustAst_Import():
return import_(_that);case RustAst_VariableDecl():
return variableDecl(_that);case RustAst_FunctionDecl():
return functionDecl(_that);case RustAst_FunctionImpl():
return functionImpl(_that);case RustAst_FunctionCall():
return functionCall(_that);case RustAst_StructDecl():
return structDecl(_that);case RustAst_RawStruct():
return rawStruct(_that);case RustAst_EnumDecl():
return enumDecl(_that);case RustAst_RawEnum():
return rawEnum(_that);case RustAst_ZeImpl():
return zeImpl(_that);case RustAst_IfConditional():
return ifConditional(_that);case RustAst_IfLet():
return ifLet(_that);case RustAst_ForLoop():
return forLoop(_that);case RustAst_WhileLoop():
return whileLoop(_that);case RustAst_WhileLet():
return whileLet(_that);case RustAst_TraitDecl():
return traitDecl(_that);case RustAst_Unknown():
return unknown(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( RustAst_Import value)?  import_,TResult? Function( RustAst_VariableDecl value)?  variableDecl,TResult? Function( RustAst_FunctionDecl value)?  functionDecl,TResult? Function( RustAst_FunctionImpl value)?  functionImpl,TResult? Function( RustAst_FunctionCall value)?  functionCall,TResult? Function( RustAst_StructDecl value)?  structDecl,TResult? Function( RustAst_RawStruct value)?  rawStruct,TResult? Function( RustAst_EnumDecl value)?  enumDecl,TResult? Function( RustAst_RawEnum value)?  rawEnum,TResult? Function( RustAst_ZeImpl value)?  zeImpl,TResult? Function( RustAst_IfConditional value)?  ifConditional,TResult? Function( RustAst_IfLet value)?  ifLet,TResult? Function( RustAst_ForLoop value)?  forLoop,TResult? Function( RustAst_WhileLoop value)?  whileLoop,TResult? Function( RustAst_WhileLet value)?  whileLet,TResult? Function( RustAst_TraitDecl value)?  traitDecl,TResult? Function( RustAst_Unknown value)?  unknown,}){
final _that = this;
switch (_that) {
case RustAst_Import() when import_ != null:
return import_(_that);case RustAst_VariableDecl() when variableDecl != null:
return variableDecl(_that);case RustAst_FunctionDecl() when functionDecl != null:
return functionDecl(_that);case RustAst_FunctionImpl() when functionImpl != null:
return functionImpl(_that);case RustAst_FunctionCall() when functionCall != null:
return functionCall(_that);case RustAst_StructDecl() when structDecl != null:
return structDecl(_that);case RustAst_RawStruct() when rawStruct != null:
return rawStruct(_that);case RustAst_EnumDecl() when enumDecl != null:
return enumDecl(_that);case RustAst_RawEnum() when rawEnum != null:
return rawEnum(_that);case RustAst_ZeImpl() when zeImpl != null:
return zeImpl(_that);case RustAst_IfConditional() when ifConditional != null:
return ifConditional(_that);case RustAst_IfLet() when ifLet != null:
return ifLet(_that);case RustAst_ForLoop() when forLoop != null:
return forLoop(_that);case RustAst_WhileLoop() when whileLoop != null:
return whileLoop(_that);case RustAst_WhileLet() when whileLet != null:
return whileLet(_that);case RustAst_TraitDecl() when traitDecl != null:
return traitDecl(_that);case RustAst_Unknown() when unknown != null:
return unknown(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( List<String> main,  List<String> final_)?  import_,TResult Function( bool static_,  bool const_,  bool pub,  String name,  String? type,  List<RustAst> value)?  variableDecl,TResult Function( bool pub,  String name,  List<(String, String?,)> params,  String return_)?  functionDecl,TResult Function( bool pub,  String name,  List<(String, String?,)> params,  String return_,  List<RustAst> children)?  functionImpl,TResult Function( String name,  List<String> params)?  functionCall,TResult Function( String name,  List<(String, String,)> chidlren)?  structDecl,TResult Function( String name,  List<(String, String?,)> chidlren)?  rawStruct,TResult Function( List<String> enumValue,  List<(String, List<String>,)> tupleTagged,  List<(String, List<(String, String,)>,)> structTagged)?  enumDecl,TResult Function( String name,  List<String>? tupleTagged,  List<(String, String?,)>? structTagged)?  rawEnum,TResult Function( String name,  String? for_,  List<RustAst> children)?  zeImpl,TResult Function( List<String> condition,  List<RustAst> children,  RustAst else_)?  ifConditional,TResult Function( String variant,  String variable,  String enum_,  List<RustAst> children,  RustAst else_)?  ifLet,TResult Function( List<String> variables,  List<String> source)?  forLoop,TResult Function( List<String> condition,  List<RustAst> children)?  whileLoop,TResult Function( String variant,  String variable,  String enum_,  List<RustAst> children)?  whileLet,TResult Function( bool pub,  List<RustAst> declarations)?  traitDecl,TResult Function()?  unknown,required TResult orElse(),}) {final _that = this;
switch (_that) {
case RustAst_Import() when import_ != null:
return import_(_that.main,_that.final_);case RustAst_VariableDecl() when variableDecl != null:
return variableDecl(_that.static_,_that.const_,_that.pub,_that.name,_that.type,_that.value);case RustAst_FunctionDecl() when functionDecl != null:
return functionDecl(_that.pub,_that.name,_that.params,_that.return_);case RustAst_FunctionImpl() when functionImpl != null:
return functionImpl(_that.pub,_that.name,_that.params,_that.return_,_that.children);case RustAst_FunctionCall() when functionCall != null:
return functionCall(_that.name,_that.params);case RustAst_StructDecl() when structDecl != null:
return structDecl(_that.name,_that.chidlren);case RustAst_RawStruct() when rawStruct != null:
return rawStruct(_that.name,_that.chidlren);case RustAst_EnumDecl() when enumDecl != null:
return enumDecl(_that.enumValue,_that.tupleTagged,_that.structTagged);case RustAst_RawEnum() when rawEnum != null:
return rawEnum(_that.name,_that.tupleTagged,_that.structTagged);case RustAst_ZeImpl() when zeImpl != null:
return zeImpl(_that.name,_that.for_,_that.children);case RustAst_IfConditional() when ifConditional != null:
return ifConditional(_that.condition,_that.children,_that.else_);case RustAst_IfLet() when ifLet != null:
return ifLet(_that.variant,_that.variable,_that.enum_,_that.children,_that.else_);case RustAst_ForLoop() when forLoop != null:
return forLoop(_that.variables,_that.source);case RustAst_WhileLoop() when whileLoop != null:
return whileLoop(_that.condition,_that.children);case RustAst_WhileLet() when whileLet != null:
return whileLet(_that.variant,_that.variable,_that.enum_,_that.children);case RustAst_TraitDecl() when traitDecl != null:
return traitDecl(_that.pub,_that.declarations);case RustAst_Unknown() when unknown != null:
return unknown();case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( List<String> main,  List<String> final_)  import_,required TResult Function( bool static_,  bool const_,  bool pub,  String name,  String? type,  List<RustAst> value)  variableDecl,required TResult Function( bool pub,  String name,  List<(String, String?,)> params,  String return_)  functionDecl,required TResult Function( bool pub,  String name,  List<(String, String?,)> params,  String return_,  List<RustAst> children)  functionImpl,required TResult Function( String name,  List<String> params)  functionCall,required TResult Function( String name,  List<(String, String,)> chidlren)  structDecl,required TResult Function( String name,  List<(String, String?,)> chidlren)  rawStruct,required TResult Function( List<String> enumValue,  List<(String, List<String>,)> tupleTagged,  List<(String, List<(String, String,)>,)> structTagged)  enumDecl,required TResult Function( String name,  List<String>? tupleTagged,  List<(String, String?,)>? structTagged)  rawEnum,required TResult Function( String name,  String? for_,  List<RustAst> children)  zeImpl,required TResult Function( List<String> condition,  List<RustAst> children,  RustAst else_)  ifConditional,required TResult Function( String variant,  String variable,  String enum_,  List<RustAst> children,  RustAst else_)  ifLet,required TResult Function( List<String> variables,  List<String> source)  forLoop,required TResult Function( List<String> condition,  List<RustAst> children)  whileLoop,required TResult Function( String variant,  String variable,  String enum_,  List<RustAst> children)  whileLet,required TResult Function( bool pub,  List<RustAst> declarations)  traitDecl,required TResult Function()  unknown,}) {final _that = this;
switch (_that) {
case RustAst_Import():
return import_(_that.main,_that.final_);case RustAst_VariableDecl():
return variableDecl(_that.static_,_that.const_,_that.pub,_that.name,_that.type,_that.value);case RustAst_FunctionDecl():
return functionDecl(_that.pub,_that.name,_that.params,_that.return_);case RustAst_FunctionImpl():
return functionImpl(_that.pub,_that.name,_that.params,_that.return_,_that.children);case RustAst_FunctionCall():
return functionCall(_that.name,_that.params);case RustAst_StructDecl():
return structDecl(_that.name,_that.chidlren);case RustAst_RawStruct():
return rawStruct(_that.name,_that.chidlren);case RustAst_EnumDecl():
return enumDecl(_that.enumValue,_that.tupleTagged,_that.structTagged);case RustAst_RawEnum():
return rawEnum(_that.name,_that.tupleTagged,_that.structTagged);case RustAst_ZeImpl():
return zeImpl(_that.name,_that.for_,_that.children);case RustAst_IfConditional():
return ifConditional(_that.condition,_that.children,_that.else_);case RustAst_IfLet():
return ifLet(_that.variant,_that.variable,_that.enum_,_that.children,_that.else_);case RustAst_ForLoop():
return forLoop(_that.variables,_that.source);case RustAst_WhileLoop():
return whileLoop(_that.condition,_that.children);case RustAst_WhileLet():
return whileLet(_that.variant,_that.variable,_that.enum_,_that.children);case RustAst_TraitDecl():
return traitDecl(_that.pub,_that.declarations);case RustAst_Unknown():
return unknown();}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( List<String> main,  List<String> final_)?  import_,TResult? Function( bool static_,  bool const_,  bool pub,  String name,  String? type,  List<RustAst> value)?  variableDecl,TResult? Function( bool pub,  String name,  List<(String, String?,)> params,  String return_)?  functionDecl,TResult? Function( bool pub,  String name,  List<(String, String?,)> params,  String return_,  List<RustAst> children)?  functionImpl,TResult? Function( String name,  List<String> params)?  functionCall,TResult? Function( String name,  List<(String, String,)> chidlren)?  structDecl,TResult? Function( String name,  List<(String, String?,)> chidlren)?  rawStruct,TResult? Function( List<String> enumValue,  List<(String, List<String>,)> tupleTagged,  List<(String, List<(String, String,)>,)> structTagged)?  enumDecl,TResult? Function( String name,  List<String>? tupleTagged,  List<(String, String?,)>? structTagged)?  rawEnum,TResult? Function( String name,  String? for_,  List<RustAst> children)?  zeImpl,TResult? Function( List<String> condition,  List<RustAst> children,  RustAst else_)?  ifConditional,TResult? Function( String variant,  String variable,  String enum_,  List<RustAst> children,  RustAst else_)?  ifLet,TResult? Function( List<String> variables,  List<String> source)?  forLoop,TResult? Function( List<String> condition,  List<RustAst> children)?  whileLoop,TResult? Function( String variant,  String variable,  String enum_,  List<RustAst> children)?  whileLet,TResult? Function( bool pub,  List<RustAst> declarations)?  traitDecl,TResult? Function()?  unknown,}) {final _that = this;
switch (_that) {
case RustAst_Import() when import_ != null:
return import_(_that.main,_that.final_);case RustAst_VariableDecl() when variableDecl != null:
return variableDecl(_that.static_,_that.const_,_that.pub,_that.name,_that.type,_that.value);case RustAst_FunctionDecl() when functionDecl != null:
return functionDecl(_that.pub,_that.name,_that.params,_that.return_);case RustAst_FunctionImpl() when functionImpl != null:
return functionImpl(_that.pub,_that.name,_that.params,_that.return_,_that.children);case RustAst_FunctionCall() when functionCall != null:
return functionCall(_that.name,_that.params);case RustAst_StructDecl() when structDecl != null:
return structDecl(_that.name,_that.chidlren);case RustAst_RawStruct() when rawStruct != null:
return rawStruct(_that.name,_that.chidlren);case RustAst_EnumDecl() when enumDecl != null:
return enumDecl(_that.enumValue,_that.tupleTagged,_that.structTagged);case RustAst_RawEnum() when rawEnum != null:
return rawEnum(_that.name,_that.tupleTagged,_that.structTagged);case RustAst_ZeImpl() when zeImpl != null:
return zeImpl(_that.name,_that.for_,_that.children);case RustAst_IfConditional() when ifConditional != null:
return ifConditional(_that.condition,_that.children,_that.else_);case RustAst_IfLet() when ifLet != null:
return ifLet(_that.variant,_that.variable,_that.enum_,_that.children,_that.else_);case RustAst_ForLoop() when forLoop != null:
return forLoop(_that.variables,_that.source);case RustAst_WhileLoop() when whileLoop != null:
return whileLoop(_that.condition,_that.children);case RustAst_WhileLet() when whileLet != null:
return whileLet(_that.variant,_that.variable,_that.enum_,_that.children);case RustAst_TraitDecl() when traitDecl != null:
return traitDecl(_that.pub,_that.declarations);case RustAst_Unknown() when unknown != null:
return unknown();case _:
  return null;

}
}

}

/// @nodoc


class RustAst_Import extends RustAst {
  const RustAst_Import({required  List<String> main, required  List<String> final_}): _main = main,_final_ = final_,super._();
  

 final  List<String> _main;
 List<String> get main {
  if (_main is EqualUnmodifiableListView) return _main;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_main);
}

 final  List<String> _final_;
 List<String> get final_ {
  if (_final_ is EqualUnmodifiableListView) return _final_;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_final_);
}


/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RustAst_ImportCopyWith<RustAst_Import> get copyWith => _$RustAst_ImportCopyWithImpl<RustAst_Import>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is RustAst_Import&&const DeepCollectionEquality().equals(other.main, _main)&&const DeepCollectionEquality().equals(other.final_, _final_));
}


@override
int get hashCode {
    return Object.hash(runtimeType,const DeepCollectionEquality().hash(_main),const DeepCollectionEquality().hash(_final_));
}

@override
String toString() {
    return 'RustAst.import_(main: $main, final_: $final_)';
}


}

/// @nodoc
abstract mixin class $RustAst_ImportCopyWith<$Res> implements $RustAstCopyWith<$Res> {
  factory $RustAst_ImportCopyWith(RustAst_Import value, $Res Function(RustAst_Import) _then) = _$RustAst_ImportCopyWithImpl;
@useResult
$Res call({
 List<String> main, List<String> final_
});




}
/// @nodoc
class _$RustAst_ImportCopyWithImpl<$Res>
    implements $RustAst_ImportCopyWith<$Res> {
  _$RustAst_ImportCopyWithImpl(this._self, this._then);

  final RustAst_Import _self;
  final $Res Function(RustAst_Import) _then;

/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? main = null,Object? final_ = null,}) {
  return _then(RustAst_Import(
main: null == main ? _self._main : main // ignore: cast_nullable_to_non_nullable
as List<String>,final_: null == final_ ? _self._final_ : final_ // ignore: cast_nullable_to_non_nullable
as List<String>,
  ));
}


}

/// @nodoc


class RustAst_VariableDecl extends RustAst {
  const RustAst_VariableDecl({required this.static_, required this.const_, required this.pub, required this.name, this.type, required  List<RustAst> value}): _value = value,super._();
  

 final  bool static_;
 final  bool const_;
 final  bool pub;
 final  String name;
 final  String? type;
 final  List<RustAst> _value;
 List<RustAst> get value {
  if (_value is EqualUnmodifiableListView) return _value;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_value);
}


/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RustAst_VariableDeclCopyWith<RustAst_VariableDecl> get copyWith => _$RustAst_VariableDeclCopyWithImpl<RustAst_VariableDecl>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is RustAst_VariableDecl&&(identical(other.static_, static_) || other.static_ == static_)&&(identical(other.const_, const_) || other.const_ == const_)&&(identical(other.pub, pub) || other.pub == pub)&&(identical(other.name, name) || other.name == name)&&(identical(other.type, type) || other.type == type)&&const DeepCollectionEquality().equals(other.value, _value));
}


@override
int get hashCode {
    return Object.hash(runtimeType,static_,const_,pub,name,type,const DeepCollectionEquality().hash(_value));
}

@override
String toString() {
    return 'RustAst.variableDecl(static_: $static_, const_: $const_, pub: $pub, name: $name, type: $type, value: $value)';
}


}

/// @nodoc
abstract mixin class $RustAst_VariableDeclCopyWith<$Res> implements $RustAstCopyWith<$Res> {
  factory $RustAst_VariableDeclCopyWith(RustAst_VariableDecl value, $Res Function(RustAst_VariableDecl) _then) = _$RustAst_VariableDeclCopyWithImpl;
@useResult
$Res call({
 bool static_, bool const_, bool pub, String name, String? type, List<RustAst> value
});




}
/// @nodoc
class _$RustAst_VariableDeclCopyWithImpl<$Res>
    implements $RustAst_VariableDeclCopyWith<$Res> {
  _$RustAst_VariableDeclCopyWithImpl(this._self, this._then);

  final RustAst_VariableDecl _self;
  final $Res Function(RustAst_VariableDecl) _then;

/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? static_ = null,Object? const_ = null,Object? pub = null,Object? name = null,Object? type = freezed,Object? value = null,}) {
  return _then(RustAst_VariableDecl(
static_: null == static_ ? _self.static_ : static_ // ignore: cast_nullable_to_non_nullable
as bool,const_: null == const_ ? _self.const_ : const_ // ignore: cast_nullable_to_non_nullable
as bool,pub: null == pub ? _self.pub : pub // ignore: cast_nullable_to_non_nullable
as bool,name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,type: freezed == type ? _self.type : type // ignore: cast_nullable_to_non_nullable
as String?,value: null == value ? _self._value : value // ignore: cast_nullable_to_non_nullable
as List<RustAst>,
  ));
}


}

/// @nodoc


class RustAst_FunctionDecl extends RustAst {
  const RustAst_FunctionDecl({required this.pub, required this.name, required  List<(String, String?,)> params, required this.return_}): _params = params,super._();
  

 final  bool pub;
 final  String name;
 final  List<(String, String?,)> _params;
 List<(String, String?,)> get params {
  if (_params is EqualUnmodifiableListView) return _params;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_params);
}

 final  String return_;

/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RustAst_FunctionDeclCopyWith<RustAst_FunctionDecl> get copyWith => _$RustAst_FunctionDeclCopyWithImpl<RustAst_FunctionDecl>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is RustAst_FunctionDecl&&(identical(other.pub, pub) || other.pub == pub)&&(identical(other.name, name) || other.name == name)&&const DeepCollectionEquality().equals(other.params, _params)&&(identical(other.return_, return_) || other.return_ == return_));
}


@override
int get hashCode {
    return Object.hash(runtimeType,pub,name,const DeepCollectionEquality().hash(_params),return_);
}

@override
String toString() {
    return 'RustAst.functionDecl(pub: $pub, name: $name, params: $params, return_: $return_)';
}


}

/// @nodoc
abstract mixin class $RustAst_FunctionDeclCopyWith<$Res> implements $RustAstCopyWith<$Res> {
  factory $RustAst_FunctionDeclCopyWith(RustAst_FunctionDecl value, $Res Function(RustAst_FunctionDecl) _then) = _$RustAst_FunctionDeclCopyWithImpl;
@useResult
$Res call({
 bool pub, String name, List<(String, String?,)> params, String return_
});




}
/// @nodoc
class _$RustAst_FunctionDeclCopyWithImpl<$Res>
    implements $RustAst_FunctionDeclCopyWith<$Res> {
  _$RustAst_FunctionDeclCopyWithImpl(this._self, this._then);

  final RustAst_FunctionDecl _self;
  final $Res Function(RustAst_FunctionDecl) _then;

/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? pub = null,Object? name = null,Object? params = null,Object? return_ = null,}) {
  return _then(RustAst_FunctionDecl(
pub: null == pub ? _self.pub : pub // ignore: cast_nullable_to_non_nullable
as bool,name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,params: null == params ? _self._params : params // ignore: cast_nullable_to_non_nullable
as List<(String, String?,)>,return_: null == return_ ? _self.return_ : return_ // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class RustAst_FunctionImpl extends RustAst {
  const RustAst_FunctionImpl({required this.pub, required this.name, required  List<(String, String?,)> params, required this.return_, required  List<RustAst> children}): _params = params,_children = children,super._();
  

 final  bool pub;
 final  String name;
 final  List<(String, String?,)> _params;
 List<(String, String?,)> get params {
  if (_params is EqualUnmodifiableListView) return _params;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_params);
}

 final  String return_;
 final  List<RustAst> _children;
 List<RustAst> get children {
  if (_children is EqualUnmodifiableListView) return _children;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_children);
}


/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RustAst_FunctionImplCopyWith<RustAst_FunctionImpl> get copyWith => _$RustAst_FunctionImplCopyWithImpl<RustAst_FunctionImpl>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is RustAst_FunctionImpl&&(identical(other.pub, pub) || other.pub == pub)&&(identical(other.name, name) || other.name == name)&&const DeepCollectionEquality().equals(other.params, _params)&&(identical(other.return_, return_) || other.return_ == return_)&&const DeepCollectionEquality().equals(other.children, _children));
}


@override
int get hashCode {
    return Object.hash(runtimeType,pub,name,const DeepCollectionEquality().hash(_params),return_,const DeepCollectionEquality().hash(_children));
}

@override
String toString() {
    return 'RustAst.functionImpl(pub: $pub, name: $name, params: $params, return_: $return_, children: $children)';
}


}

/// @nodoc
abstract mixin class $RustAst_FunctionImplCopyWith<$Res> implements $RustAstCopyWith<$Res> {
  factory $RustAst_FunctionImplCopyWith(RustAst_FunctionImpl value, $Res Function(RustAst_FunctionImpl) _then) = _$RustAst_FunctionImplCopyWithImpl;
@useResult
$Res call({
 bool pub, String name, List<(String, String?,)> params, String return_, List<RustAst> children
});




}
/// @nodoc
class _$RustAst_FunctionImplCopyWithImpl<$Res>
    implements $RustAst_FunctionImplCopyWith<$Res> {
  _$RustAst_FunctionImplCopyWithImpl(this._self, this._then);

  final RustAst_FunctionImpl _self;
  final $Res Function(RustAst_FunctionImpl) _then;

/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? pub = null,Object? name = null,Object? params = null,Object? return_ = null,Object? children = null,}) {
  return _then(RustAst_FunctionImpl(
pub: null == pub ? _self.pub : pub // ignore: cast_nullable_to_non_nullable
as bool,name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,params: null == params ? _self._params : params // ignore: cast_nullable_to_non_nullable
as List<(String, String?,)>,return_: null == return_ ? _self.return_ : return_ // ignore: cast_nullable_to_non_nullable
as String,children: null == children ? _self._children : children // ignore: cast_nullable_to_non_nullable
as List<RustAst>,
  ));
}


}

/// @nodoc


class RustAst_FunctionCall extends RustAst {
  const RustAst_FunctionCall({required this.name, required  List<String> params}): _params = params,super._();
  

 final  String name;
 final  List<String> _params;
 List<String> get params {
  if (_params is EqualUnmodifiableListView) return _params;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_params);
}


/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RustAst_FunctionCallCopyWith<RustAst_FunctionCall> get copyWith => _$RustAst_FunctionCallCopyWithImpl<RustAst_FunctionCall>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is RustAst_FunctionCall&&(identical(other.name, name) || other.name == name)&&const DeepCollectionEquality().equals(other.params, _params));
}


@override
int get hashCode {
    return Object.hash(runtimeType,name,const DeepCollectionEquality().hash(_params));
}

@override
String toString() {
    return 'RustAst.functionCall(name: $name, params: $params)';
}


}

/// @nodoc
abstract mixin class $RustAst_FunctionCallCopyWith<$Res> implements $RustAstCopyWith<$Res> {
  factory $RustAst_FunctionCallCopyWith(RustAst_FunctionCall value, $Res Function(RustAst_FunctionCall) _then) = _$RustAst_FunctionCallCopyWithImpl;
@useResult
$Res call({
 String name, List<String> params
});




}
/// @nodoc
class _$RustAst_FunctionCallCopyWithImpl<$Res>
    implements $RustAst_FunctionCallCopyWith<$Res> {
  _$RustAst_FunctionCallCopyWithImpl(this._self, this._then);

  final RustAst_FunctionCall _self;
  final $Res Function(RustAst_FunctionCall) _then;

/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? name = null,Object? params = null,}) {
  return _then(RustAst_FunctionCall(
name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,params: null == params ? _self._params : params // ignore: cast_nullable_to_non_nullable
as List<String>,
  ));
}


}

/// @nodoc


class RustAst_StructDecl extends RustAst {
  const RustAst_StructDecl({required this.name, required  List<(String, String,)> chidlren}): _chidlren = chidlren,super._();
  

 final  String name;
 final  List<(String, String,)> _chidlren;
 List<(String, String,)> get chidlren {
  if (_chidlren is EqualUnmodifiableListView) return _chidlren;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_chidlren);
}


/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RustAst_StructDeclCopyWith<RustAst_StructDecl> get copyWith => _$RustAst_StructDeclCopyWithImpl<RustAst_StructDecl>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is RustAst_StructDecl&&(identical(other.name, name) || other.name == name)&&const DeepCollectionEquality().equals(other.chidlren, _chidlren));
}


@override
int get hashCode {
    return Object.hash(runtimeType,name,const DeepCollectionEquality().hash(_chidlren));
}

@override
String toString() {
    return 'RustAst.structDecl(name: $name, chidlren: $chidlren)';
}


}

/// @nodoc
abstract mixin class $RustAst_StructDeclCopyWith<$Res> implements $RustAstCopyWith<$Res> {
  factory $RustAst_StructDeclCopyWith(RustAst_StructDecl value, $Res Function(RustAst_StructDecl) _then) = _$RustAst_StructDeclCopyWithImpl;
@useResult
$Res call({
 String name, List<(String, String,)> chidlren
});




}
/// @nodoc
class _$RustAst_StructDeclCopyWithImpl<$Res>
    implements $RustAst_StructDeclCopyWith<$Res> {
  _$RustAst_StructDeclCopyWithImpl(this._self, this._then);

  final RustAst_StructDecl _self;
  final $Res Function(RustAst_StructDecl) _then;

/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? name = null,Object? chidlren = null,}) {
  return _then(RustAst_StructDecl(
name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,chidlren: null == chidlren ? _self._chidlren : chidlren // ignore: cast_nullable_to_non_nullable
as List<(String, String,)>,
  ));
}


}

/// @nodoc


class RustAst_RawStruct extends RustAst {
  const RustAst_RawStruct({required this.name, required  List<(String, String?,)> chidlren}): _chidlren = chidlren,super._();
  

 final  String name;
 final  List<(String, String?,)> _chidlren;
 List<(String, String?,)> get chidlren {
  if (_chidlren is EqualUnmodifiableListView) return _chidlren;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_chidlren);
}


/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RustAst_RawStructCopyWith<RustAst_RawStruct> get copyWith => _$RustAst_RawStructCopyWithImpl<RustAst_RawStruct>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is RustAst_RawStruct&&(identical(other.name, name) || other.name == name)&&const DeepCollectionEquality().equals(other.chidlren, _chidlren));
}


@override
int get hashCode {
    return Object.hash(runtimeType,name,const DeepCollectionEquality().hash(_chidlren));
}

@override
String toString() {
    return 'RustAst.rawStruct(name: $name, chidlren: $chidlren)';
}


}

/// @nodoc
abstract mixin class $RustAst_RawStructCopyWith<$Res> implements $RustAstCopyWith<$Res> {
  factory $RustAst_RawStructCopyWith(RustAst_RawStruct value, $Res Function(RustAst_RawStruct) _then) = _$RustAst_RawStructCopyWithImpl;
@useResult
$Res call({
 String name, List<(String, String?,)> chidlren
});




}
/// @nodoc
class _$RustAst_RawStructCopyWithImpl<$Res>
    implements $RustAst_RawStructCopyWith<$Res> {
  _$RustAst_RawStructCopyWithImpl(this._self, this._then);

  final RustAst_RawStruct _self;
  final $Res Function(RustAst_RawStruct) _then;

/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? name = null,Object? chidlren = null,}) {
  return _then(RustAst_RawStruct(
name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,chidlren: null == chidlren ? _self._chidlren : chidlren // ignore: cast_nullable_to_non_nullable
as List<(String, String?,)>,
  ));
}


}

/// @nodoc


class RustAst_EnumDecl extends RustAst {
  const RustAst_EnumDecl({required  List<String> enumValue, required  List<(String, List<String>,)> tupleTagged, required  List<(String, List<(String, String,)>,)> structTagged}): _enumValue = enumValue,_tupleTagged = tupleTagged,_structTagged = structTagged,super._();
  

 final  List<String> _enumValue;
 List<String> get enumValue {
  if (_enumValue is EqualUnmodifiableListView) return _enumValue;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_enumValue);
}

 final  List<(String, List<String>,)> _tupleTagged;
 List<(String, List<String>,)> get tupleTagged {
  if (_tupleTagged is EqualUnmodifiableListView) return _tupleTagged;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_tupleTagged);
}

 final  List<(String, List<(String, String,)>,)> _structTagged;
 List<(String, List<(String, String,)>,)> get structTagged {
  if (_structTagged is EqualUnmodifiableListView) return _structTagged;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_structTagged);
}


/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RustAst_EnumDeclCopyWith<RustAst_EnumDecl> get copyWith => _$RustAst_EnumDeclCopyWithImpl<RustAst_EnumDecl>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is RustAst_EnumDecl&&const DeepCollectionEquality().equals(other.enumValue, _enumValue)&&const DeepCollectionEquality().equals(other.tupleTagged, _tupleTagged)&&const DeepCollectionEquality().equals(other.structTagged, _structTagged));
}


@override
int get hashCode {
    return Object.hash(runtimeType,const DeepCollectionEquality().hash(_enumValue),const DeepCollectionEquality().hash(_tupleTagged),const DeepCollectionEquality().hash(_structTagged));
}

@override
String toString() {
    return 'RustAst.enumDecl(enumValue: $enumValue, tupleTagged: $tupleTagged, structTagged: $structTagged)';
}


}

/// @nodoc
abstract mixin class $RustAst_EnumDeclCopyWith<$Res> implements $RustAstCopyWith<$Res> {
  factory $RustAst_EnumDeclCopyWith(RustAst_EnumDecl value, $Res Function(RustAst_EnumDecl) _then) = _$RustAst_EnumDeclCopyWithImpl;
@useResult
$Res call({
 List<String> enumValue, List<(String, List<String>,)> tupleTagged, List<(String, List<(String, String,)>,)> structTagged
});




}
/// @nodoc
class _$RustAst_EnumDeclCopyWithImpl<$Res>
    implements $RustAst_EnumDeclCopyWith<$Res> {
  _$RustAst_EnumDeclCopyWithImpl(this._self, this._then);

  final RustAst_EnumDecl _self;
  final $Res Function(RustAst_EnumDecl) _then;

/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? enumValue = null,Object? tupleTagged = null,Object? structTagged = null,}) {
  return _then(RustAst_EnumDecl(
enumValue: null == enumValue ? _self._enumValue : enumValue // ignore: cast_nullable_to_non_nullable
as List<String>,tupleTagged: null == tupleTagged ? _self._tupleTagged : tupleTagged // ignore: cast_nullable_to_non_nullable
as List<(String, List<String>,)>,structTagged: null == structTagged ? _self._structTagged : structTagged // ignore: cast_nullable_to_non_nullable
as List<(String, List<(String, String,)>,)>,
  ));
}


}

/// @nodoc


class RustAst_RawEnum extends RustAst {
  const RustAst_RawEnum({required this.name,  List<String>? tupleTagged,  List<(String, String?,)>? structTagged}): _tupleTagged = tupleTagged,_structTagged = structTagged,super._();
  

 final  String name;
 final  List<String>? _tupleTagged;
 List<String>? get tupleTagged {
  final value = _tupleTagged;
  if (value == null) return null;
  if (_tupleTagged is EqualUnmodifiableListView) return _tupleTagged;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(value);
}

 final  List<(String, String?,)>? _structTagged;
 List<(String, String?,)>? get structTagged {
  final value = _structTagged;
  if (value == null) return null;
  if (_structTagged is EqualUnmodifiableListView) return _structTagged;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(value);
}


/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RustAst_RawEnumCopyWith<RustAst_RawEnum> get copyWith => _$RustAst_RawEnumCopyWithImpl<RustAst_RawEnum>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is RustAst_RawEnum&&(identical(other.name, name) || other.name == name)&&const DeepCollectionEquality().equals(other.tupleTagged, _tupleTagged)&&const DeepCollectionEquality().equals(other.structTagged, _structTagged));
}


@override
int get hashCode {
    return Object.hash(runtimeType,name,const DeepCollectionEquality().hash(_tupleTagged),const DeepCollectionEquality().hash(_structTagged));
}

@override
String toString() {
    return 'RustAst.rawEnum(name: $name, tupleTagged: $tupleTagged, structTagged: $structTagged)';
}


}

/// @nodoc
abstract mixin class $RustAst_RawEnumCopyWith<$Res> implements $RustAstCopyWith<$Res> {
  factory $RustAst_RawEnumCopyWith(RustAst_RawEnum value, $Res Function(RustAst_RawEnum) _then) = _$RustAst_RawEnumCopyWithImpl;
@useResult
$Res call({
 String name, List<String>? tupleTagged, List<(String, String?,)>? structTagged
});




}
/// @nodoc
class _$RustAst_RawEnumCopyWithImpl<$Res>
    implements $RustAst_RawEnumCopyWith<$Res> {
  _$RustAst_RawEnumCopyWithImpl(this._self, this._then);

  final RustAst_RawEnum _self;
  final $Res Function(RustAst_RawEnum) _then;

/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? name = null,Object? tupleTagged = freezed,Object? structTagged = freezed,}) {
  return _then(RustAst_RawEnum(
name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,tupleTagged: freezed == tupleTagged ? _self._tupleTagged : tupleTagged // ignore: cast_nullable_to_non_nullable
as List<String>?,structTagged: freezed == structTagged ? _self._structTagged : structTagged // ignore: cast_nullable_to_non_nullable
as List<(String, String?,)>?,
  ));
}


}

/// @nodoc


class RustAst_ZeImpl extends RustAst {
  const RustAst_ZeImpl({required this.name, this.for_, required  List<RustAst> children}): _children = children,super._();
  

 final  String name;
 final  String? for_;
 final  List<RustAst> _children;
 List<RustAst> get children {
  if (_children is EqualUnmodifiableListView) return _children;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_children);
}


/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RustAst_ZeImplCopyWith<RustAst_ZeImpl> get copyWith => _$RustAst_ZeImplCopyWithImpl<RustAst_ZeImpl>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is RustAst_ZeImpl&&(identical(other.name, name) || other.name == name)&&(identical(other.for_, for_) || other.for_ == for_)&&const DeepCollectionEquality().equals(other.children, _children));
}


@override
int get hashCode {
    return Object.hash(runtimeType,name,for_,const DeepCollectionEquality().hash(_children));
}

@override
String toString() {
    return 'RustAst.zeImpl(name: $name, for_: $for_, children: $children)';
}


}

/// @nodoc
abstract mixin class $RustAst_ZeImplCopyWith<$Res> implements $RustAstCopyWith<$Res> {
  factory $RustAst_ZeImplCopyWith(RustAst_ZeImpl value, $Res Function(RustAst_ZeImpl) _then) = _$RustAst_ZeImplCopyWithImpl;
@useResult
$Res call({
 String name, String? for_, List<RustAst> children
});




}
/// @nodoc
class _$RustAst_ZeImplCopyWithImpl<$Res>
    implements $RustAst_ZeImplCopyWith<$Res> {
  _$RustAst_ZeImplCopyWithImpl(this._self, this._then);

  final RustAst_ZeImpl _self;
  final $Res Function(RustAst_ZeImpl) _then;

/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? name = null,Object? for_ = freezed,Object? children = null,}) {
  return _then(RustAst_ZeImpl(
name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,for_: freezed == for_ ? _self.for_ : for_ // ignore: cast_nullable_to_non_nullable
as String?,children: null == children ? _self._children : children // ignore: cast_nullable_to_non_nullable
as List<RustAst>,
  ));
}


}

/// @nodoc


class RustAst_IfConditional extends RustAst {
  const RustAst_IfConditional({required  List<String> condition, required  List<RustAst> children, required this.else_}): _condition = condition,_children = children,super._();
  

 final  List<String> _condition;
 List<String> get condition {
  if (_condition is EqualUnmodifiableListView) return _condition;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_condition);
}

 final  List<RustAst> _children;
 List<RustAst> get children {
  if (_children is EqualUnmodifiableListView) return _children;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_children);
}

 final  RustAst else_;

/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RustAst_IfConditionalCopyWith<RustAst_IfConditional> get copyWith => _$RustAst_IfConditionalCopyWithImpl<RustAst_IfConditional>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is RustAst_IfConditional&&const DeepCollectionEquality().equals(other.condition, _condition)&&const DeepCollectionEquality().equals(other.children, _children)&&(identical(other.else_, else_) || other.else_ == else_));
}


@override
int get hashCode {
    return Object.hash(runtimeType,const DeepCollectionEquality().hash(_condition),const DeepCollectionEquality().hash(_children),else_);
}

@override
String toString() {
    return 'RustAst.ifConditional(condition: $condition, children: $children, else_: $else_)';
}


}

/// @nodoc
abstract mixin class $RustAst_IfConditionalCopyWith<$Res> implements $RustAstCopyWith<$Res> {
  factory $RustAst_IfConditionalCopyWith(RustAst_IfConditional value, $Res Function(RustAst_IfConditional) _then) = _$RustAst_IfConditionalCopyWithImpl;
@useResult
$Res call({
 List<String> condition, List<RustAst> children, RustAst else_
});


$RustAstCopyWith<$Res> get else_;

}
/// @nodoc
class _$RustAst_IfConditionalCopyWithImpl<$Res>
    implements $RustAst_IfConditionalCopyWith<$Res> {
  _$RustAst_IfConditionalCopyWithImpl(this._self, this._then);

  final RustAst_IfConditional _self;
  final $Res Function(RustAst_IfConditional) _then;

/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? condition = null,Object? children = null,Object? else_ = null,}) {
  return _then(RustAst_IfConditional(
condition: null == condition ? _self._condition : condition // ignore: cast_nullable_to_non_nullable
as List<String>,children: null == children ? _self._children : children // ignore: cast_nullable_to_non_nullable
as List<RustAst>,else_: null == else_ ? _self.else_ : else_ // ignore: cast_nullable_to_non_nullable
as RustAst,
  ));
}

/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$RustAstCopyWith<$Res> get else_ {
  
  return $RustAstCopyWith<$Res>(_self.else_, (value) {
    return _then(_self.copyWith(else_: value));
  });
}
}

/// @nodoc


class RustAst_IfLet extends RustAst {
  const RustAst_IfLet({required this.variant, required this.variable, required this.enum_, required  List<RustAst> children, required this.else_}): _children = children,super._();
  

 final  String variant;
 final  String variable;
 final  String enum_;
 final  List<RustAst> _children;
 List<RustAst> get children {
  if (_children is EqualUnmodifiableListView) return _children;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_children);
}

 final  RustAst else_;

/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RustAst_IfLetCopyWith<RustAst_IfLet> get copyWith => _$RustAst_IfLetCopyWithImpl<RustAst_IfLet>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is RustAst_IfLet&&(identical(other.variant, variant) || other.variant == variant)&&(identical(other.variable, variable) || other.variable == variable)&&(identical(other.enum_, enum_) || other.enum_ == enum_)&&const DeepCollectionEquality().equals(other.children, _children)&&(identical(other.else_, else_) || other.else_ == else_));
}


@override
int get hashCode {
    return Object.hash(runtimeType,variant,variable,enum_,const DeepCollectionEquality().hash(_children),else_);
}

@override
String toString() {
    return 'RustAst.ifLet(variant: $variant, variable: $variable, enum_: $enum_, children: $children, else_: $else_)';
}


}

/// @nodoc
abstract mixin class $RustAst_IfLetCopyWith<$Res> implements $RustAstCopyWith<$Res> {
  factory $RustAst_IfLetCopyWith(RustAst_IfLet value, $Res Function(RustAst_IfLet) _then) = _$RustAst_IfLetCopyWithImpl;
@useResult
$Res call({
 String variant, String variable, String enum_, List<RustAst> children, RustAst else_
});


$RustAstCopyWith<$Res> get else_;

}
/// @nodoc
class _$RustAst_IfLetCopyWithImpl<$Res>
    implements $RustAst_IfLetCopyWith<$Res> {
  _$RustAst_IfLetCopyWithImpl(this._self, this._then);

  final RustAst_IfLet _self;
  final $Res Function(RustAst_IfLet) _then;

/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? variant = null,Object? variable = null,Object? enum_ = null,Object? children = null,Object? else_ = null,}) {
  return _then(RustAst_IfLet(
variant: null == variant ? _self.variant : variant // ignore: cast_nullable_to_non_nullable
as String,variable: null == variable ? _self.variable : variable // ignore: cast_nullable_to_non_nullable
as String,enum_: null == enum_ ? _self.enum_ : enum_ // ignore: cast_nullable_to_non_nullable
as String,children: null == children ? _self._children : children // ignore: cast_nullable_to_non_nullable
as List<RustAst>,else_: null == else_ ? _self.else_ : else_ // ignore: cast_nullable_to_non_nullable
as RustAst,
  ));
}

/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$RustAstCopyWith<$Res> get else_ {
  
  return $RustAstCopyWith<$Res>(_self.else_, (value) {
    return _then(_self.copyWith(else_: value));
  });
}
}

/// @nodoc


class RustAst_ForLoop extends RustAst {
  const RustAst_ForLoop({required  List<String> variables, required  List<String> source}): _variables = variables,_source = source,super._();
  

 final  List<String> _variables;
 List<String> get variables {
  if (_variables is EqualUnmodifiableListView) return _variables;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_variables);
}

 final  List<String> _source;
 List<String> get source {
  if (_source is EqualUnmodifiableListView) return _source;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_source);
}


/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RustAst_ForLoopCopyWith<RustAst_ForLoop> get copyWith => _$RustAst_ForLoopCopyWithImpl<RustAst_ForLoop>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is RustAst_ForLoop&&const DeepCollectionEquality().equals(other.variables, _variables)&&const DeepCollectionEquality().equals(other.source, _source));
}


@override
int get hashCode {
    return Object.hash(runtimeType,const DeepCollectionEquality().hash(_variables),const DeepCollectionEquality().hash(_source));
}

@override
String toString() {
    return 'RustAst.forLoop(variables: $variables, source: $source)';
}


}

/// @nodoc
abstract mixin class $RustAst_ForLoopCopyWith<$Res> implements $RustAstCopyWith<$Res> {
  factory $RustAst_ForLoopCopyWith(RustAst_ForLoop value, $Res Function(RustAst_ForLoop) _then) = _$RustAst_ForLoopCopyWithImpl;
@useResult
$Res call({
 List<String> variables, List<String> source
});




}
/// @nodoc
class _$RustAst_ForLoopCopyWithImpl<$Res>
    implements $RustAst_ForLoopCopyWith<$Res> {
  _$RustAst_ForLoopCopyWithImpl(this._self, this._then);

  final RustAst_ForLoop _self;
  final $Res Function(RustAst_ForLoop) _then;

/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? variables = null,Object? source = null,}) {
  return _then(RustAst_ForLoop(
variables: null == variables ? _self._variables : variables // ignore: cast_nullable_to_non_nullable
as List<String>,source: null == source ? _self._source : source // ignore: cast_nullable_to_non_nullable
as List<String>,
  ));
}


}

/// @nodoc


class RustAst_WhileLoop extends RustAst {
  const RustAst_WhileLoop({required  List<String> condition, required  List<RustAst> children}): _condition = condition,_children = children,super._();
  

 final  List<String> _condition;
 List<String> get condition {
  if (_condition is EqualUnmodifiableListView) return _condition;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_condition);
}

 final  List<RustAst> _children;
 List<RustAst> get children {
  if (_children is EqualUnmodifiableListView) return _children;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_children);
}


/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RustAst_WhileLoopCopyWith<RustAst_WhileLoop> get copyWith => _$RustAst_WhileLoopCopyWithImpl<RustAst_WhileLoop>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is RustAst_WhileLoop&&const DeepCollectionEquality().equals(other.condition, _condition)&&const DeepCollectionEquality().equals(other.children, _children));
}


@override
int get hashCode {
    return Object.hash(runtimeType,const DeepCollectionEquality().hash(_condition),const DeepCollectionEquality().hash(_children));
}

@override
String toString() {
    return 'RustAst.whileLoop(condition: $condition, children: $children)';
}


}

/// @nodoc
abstract mixin class $RustAst_WhileLoopCopyWith<$Res> implements $RustAstCopyWith<$Res> {
  factory $RustAst_WhileLoopCopyWith(RustAst_WhileLoop value, $Res Function(RustAst_WhileLoop) _then) = _$RustAst_WhileLoopCopyWithImpl;
@useResult
$Res call({
 List<String> condition, List<RustAst> children
});




}
/// @nodoc
class _$RustAst_WhileLoopCopyWithImpl<$Res>
    implements $RustAst_WhileLoopCopyWith<$Res> {
  _$RustAst_WhileLoopCopyWithImpl(this._self, this._then);

  final RustAst_WhileLoop _self;
  final $Res Function(RustAst_WhileLoop) _then;

/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? condition = null,Object? children = null,}) {
  return _then(RustAst_WhileLoop(
condition: null == condition ? _self._condition : condition // ignore: cast_nullable_to_non_nullable
as List<String>,children: null == children ? _self._children : children // ignore: cast_nullable_to_non_nullable
as List<RustAst>,
  ));
}


}

/// @nodoc


class RustAst_WhileLet extends RustAst {
  const RustAst_WhileLet({required this.variant, required this.variable, required this.enum_, required  List<RustAst> children}): _children = children,super._();
  

 final  String variant;
 final  String variable;
 final  String enum_;
 final  List<RustAst> _children;
 List<RustAst> get children {
  if (_children is EqualUnmodifiableListView) return _children;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_children);
}


/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RustAst_WhileLetCopyWith<RustAst_WhileLet> get copyWith => _$RustAst_WhileLetCopyWithImpl<RustAst_WhileLet>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is RustAst_WhileLet&&(identical(other.variant, variant) || other.variant == variant)&&(identical(other.variable, variable) || other.variable == variable)&&(identical(other.enum_, enum_) || other.enum_ == enum_)&&const DeepCollectionEquality().equals(other.children, _children));
}


@override
int get hashCode {
    return Object.hash(runtimeType,variant,variable,enum_,const DeepCollectionEquality().hash(_children));
}

@override
String toString() {
    return 'RustAst.whileLet(variant: $variant, variable: $variable, enum_: $enum_, children: $children)';
}


}

/// @nodoc
abstract mixin class $RustAst_WhileLetCopyWith<$Res> implements $RustAstCopyWith<$Res> {
  factory $RustAst_WhileLetCopyWith(RustAst_WhileLet value, $Res Function(RustAst_WhileLet) _then) = _$RustAst_WhileLetCopyWithImpl;
@useResult
$Res call({
 String variant, String variable, String enum_, List<RustAst> children
});




}
/// @nodoc
class _$RustAst_WhileLetCopyWithImpl<$Res>
    implements $RustAst_WhileLetCopyWith<$Res> {
  _$RustAst_WhileLetCopyWithImpl(this._self, this._then);

  final RustAst_WhileLet _self;
  final $Res Function(RustAst_WhileLet) _then;

/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? variant = null,Object? variable = null,Object? enum_ = null,Object? children = null,}) {
  return _then(RustAst_WhileLet(
variant: null == variant ? _self.variant : variant // ignore: cast_nullable_to_non_nullable
as String,variable: null == variable ? _self.variable : variable // ignore: cast_nullable_to_non_nullable
as String,enum_: null == enum_ ? _self.enum_ : enum_ // ignore: cast_nullable_to_non_nullable
as String,children: null == children ? _self._children : children // ignore: cast_nullable_to_non_nullable
as List<RustAst>,
  ));
}


}

/// @nodoc


class RustAst_TraitDecl extends RustAst {
  const RustAst_TraitDecl({required this.pub, required  List<RustAst> declarations}): _declarations = declarations,super._();
  

 final  bool pub;
 final  List<RustAst> _declarations;
 List<RustAst> get declarations {
  if (_declarations is EqualUnmodifiableListView) return _declarations;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_declarations);
}


/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RustAst_TraitDeclCopyWith<RustAst_TraitDecl> get copyWith => _$RustAst_TraitDeclCopyWithImpl<RustAst_TraitDecl>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is RustAst_TraitDecl&&(identical(other.pub, pub) || other.pub == pub)&&const DeepCollectionEquality().equals(other.declarations, _declarations));
}


@override
int get hashCode {
    return Object.hash(runtimeType,pub,const DeepCollectionEquality().hash(_declarations));
}

@override
String toString() {
    return 'RustAst.traitDecl(pub: $pub, declarations: $declarations)';
}


}

/// @nodoc
abstract mixin class $RustAst_TraitDeclCopyWith<$Res> implements $RustAstCopyWith<$Res> {
  factory $RustAst_TraitDeclCopyWith(RustAst_TraitDecl value, $Res Function(RustAst_TraitDecl) _then) = _$RustAst_TraitDeclCopyWithImpl;
@useResult
$Res call({
 bool pub, List<RustAst> declarations
});




}
/// @nodoc
class _$RustAst_TraitDeclCopyWithImpl<$Res>
    implements $RustAst_TraitDeclCopyWith<$Res> {
  _$RustAst_TraitDeclCopyWithImpl(this._self, this._then);

  final RustAst_TraitDecl _self;
  final $Res Function(RustAst_TraitDecl) _then;

/// Create a copy of RustAst
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? pub = null,Object? declarations = null,}) {
  return _then(RustAst_TraitDecl(
pub: null == pub ? _self.pub : pub // ignore: cast_nullable_to_non_nullable
as bool,declarations: null == declarations ? _self._declarations : declarations // ignore: cast_nullable_to_non_nullable
as List<RustAst>,
  ));
}


}

/// @nodoc


class RustAst_Unknown extends RustAst {
  const RustAst_Unknown(): super._();
  






@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is RustAst_Unknown);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
    return 'RustAst.unknown()';
}


}




// dart format on
