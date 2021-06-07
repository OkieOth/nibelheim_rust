## Template to create Rust structs and enums from a given model
<%
    import yacg.templateHelper as templateHelper
    import yacg.model.modelFuncs as modelFuncs
    import yacg.model.model as model

    templateFile = 'rust.mako'
    templateVersion = '0.1.0'
%>/*
Attention this file is created with codeGen. Manual changes will be overwritten
with the next codeGen run.

Generated with yacg (https://github.com/OkieOth/yacg)
(template: ${templateFile} v${templateVersion})
*/

% for type in modelTypes:

    % if type.description != None:
/* 
${templateHelper.addLineBreakToDescription(type.description,0)}
*/
    % endif
    % if isinstance(type, model.EnumType):
pub enum ${modelFuncs.getTypeName(type)} {
        % for value in type.values:
    ${value},
        % endfor
}
    % else:
pub struct ${modelFuncs.getTypeName(type)} {
    // TODO
}
    % endif
% endfor
