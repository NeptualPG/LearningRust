# GearShift — Multi-Tenant Role-Based Access Control (RBAC) and Permission Management

## 1. Role and Objective

Act as a Senior Software Architect, Django Backend Engineer, React Frontend Engineer, and Application Security Specialist.

Analyze and implement a configurable authorization system for **GearShift**, a multi-tenant business management platform built with:

- **Backend:** Python, Django, Django REST Framework.
- **Frontend:** React, TypeScript.
- **Database:** PostgreSQL.
- **Architecture:** Preserve the existing layered architecture, including repositories, selectors, services, DTOs, API views, serializers, and permission classes where applicable.

The objective is to create a secure, scalable, and manageable permission system that supports multiple companies, branches, modules, roles, and employees.

**Critical instruction:** Inspect the existing codebase before making changes. Reuse existing models, authentication, company relationships, API conventions, and architectural patterns. Do not duplicate existing functionality or replace the current architecture unnecessarily.

Do not assume that hiding a frontend button provides security. All authorization decisions must be enforced by the backend.

## 2. Core Authorization Model

Implement a hybrid authorization model combining:

- **RBAC:** Role-Based Access Control.
- **Scoped permissions:** Permissions restricted to a company, branch, or module.
- **Permission overrides:** Authorized administrators can grant or revoke specific permissions for individual users.
- **Least privilege:** Users receive only the access required for their responsibilities.
- **Explicit cross-branch access:** Access to one branch must not automatically grant access to every branch.

The authorization hierarchy should support:

Company → Branch → Module → Resource → Action

Each permission assignment must identify its applicable scope.

For example:

- Company: GearShift Demo Company.
- Branch: Medellín.
- Module: Inventory.
- Resource: Products.
- Actions: View, create, update, delete, adjust stock.

A user might be allowed to view inventory in Medellín but not modify it, while being allowed to manage inventory in Bogotá.

Permissions must be independently evaluated for each relevant operation and scope.

## 3. Company and Branch Isolation

Every company must have its own isolated authorization configuration.

Requirements:

1. Users can belong to one or multiple companies only through explicit membership assignments.
2. Each membership has a status, such as active, pending, or suspended, according to existing project conventions.
3. Users can be assigned to one or multiple branches.
4. Branch access must be explicit and independently configurable.
5. A company-wide permission must be distinguishable from a permission restricted to a specific branch.
6. Access to a branch must never be inferred merely from knowing its identifier.
7. All database queries and mutations must enforce tenant isolation.
8. Prevent insecure direct object reference (IDOR) vulnerabilities and cross-company data leakage.
9. When a company or branch is suspended, its permissions must no longer authorize protected operations.
10. Users with no valid company membership must not access company resources.

Never treat a global administrator, company owner, branch manager, and ordinary employee as interchangeable concepts.

## 4. Default Company Profiles by Business Size

Create suggested authorization templates based on company size and operational complexity.

These templates are starting configurations, not rigid limits. Administrators must be able to customize them.

### A. Small Companies

Prioritize simplicity and low administrative overhead.

Suggested roles:

- **Owner / General Administrator:** Manages company configuration, all authorized branches, employees, roles, permissions, and business operations.
- **Trusted Manager:** Manages assigned operational areas and branches, subject to explicit permissions.
- **Branch Manager:** Manages authorized operations within assigned branches.
- **Employee:** Receives access only to the modules and actions required for their job.
- **Cashier / Sales Employee:** Accesses permitted sales and cash operations without automatically receiving inventory administration or company configuration privileges.

Allow a small company to operate with only a few roles. Do not force the owner to configure a complex corporate hierarchy.

### B. Medium-Sized Companies

Introduce more specialized operational responsibilities.

Suggested roles:

- Company Administrator.
- Operations Manager.
- Branch Manager.
- Inventory Manager.
- Sales Manager.
- Cashier.
- Service Manager.
- Technician.
- Purchasing Employee.
- Accountant / Financial Operator.
- Read-Only Auditor.
- Standard Employee.

Each role must have configurable module, action, and branch scopes.

For example, an inventory manager could manage inventory in two branches without gaining access to payroll, financial administration, or the other branches.

### C. Large Companies

Support organizational complexity and separation of duties.

Suggested roles:

- Company Owner.
- General Administrator.
- Regional Manager.
- Branch Manager.
- Operations Director.
- Inventory Supervisor.
- Inventory Operator.
- Sales Director.
- Sales Representative.
- Finance Manager.
- Cashier.
- Accountant.
- Service Manager.
- Technician.
- Procurement Manager.
- Human Resources Administrator, if an HR module exists.
- Compliance Officer.
- Auditor.
- Custom Roles.

Support regional or branch-group scopes only if the existing data model requires them or the implementation can introduce them without unnecessary complexity.

Large companies must be able to create and manage custom roles independently of the default templates.

### Template Configuration Rules

1. Do not use employee count alone as the authorization mechanism.
2. Company size selects an initial template, not a permanent security policy.
3. Templates must be versioned or otherwise tracked so that future changes do not silently overwrite customized roles.
4. Applying a new template to an existing company must not unexpectedly revoke or grant privileges.
5. Provide a preview of permission changes before applying a template.
6. Preserve customized roles and permissions unless an administrator explicitly approves changes.
7. Seed templates idempotently, without creating duplicate roles or permissions.

## 5. Permission Catalog

Create a centralized, extensible permission catalog.

Each permission should include:

- Stable identifier or permission code.
- Human-readable name.
- Description.
- Associated module.
- Resource or entity.
- Supported action.
- Whether it is sensitive or administrative.
- Whether it can be assigned to custom roles.
- Whether the permission requires additional restrictions.
- Active status, where appropriate.

Use stable permission codes, for example:

- `company.view`
- `company.settings.update`
- `branches.view`
- `branches.create`
- `branches.update`
- `employees.view`
- `employees.invite`
- `employees.update`
- `employees.suspend`
- `roles.view`
- `roles.create`
- `roles.update`
- `roles.assign`
- `permissions.view`
- `permissions.assign`
- `inventory.products.view`
- `inventory.products.create`
- `inventory.products.update`
- `inventory.products.delete`
- `inventory.stock.view`
- `inventory.stock.adjust`
- `sales.view`
- `sales.create`
- `sales.cancel`
- `cash.open`
- `cash.view`
- `cash.close`
- `cash.reconcile`
- `service_orders.view`
- `service_orders.create`
- `service_orders.update`
- `reports.view`
- `reports.export`

Treat this list as an initial proposal. Inspect the existing GearShift modules and generate permission definitions that correspond to real features.

Do not create permissions for modules that do not exist merely to make the catalog look complete.

Separate reading, creating, updating, deleting, approving, exporting, and administrative actions when those distinctions matter.

## 6. Authorization Data Model

Inspect the current database models and implement the minimum necessary entities.

The conceptual model should support:

### Company Membership

Associates a user with a company and defines their membership status and company-specific role assignments.

### Branch Membership or Branch Scope

Associates a company member with one or more branches, including their access status and assigned scope.

### Role

Defines a named, company-specific or system-provided collection of permissions.

Support both protected system roles and customizable company roles.

### Permission Definition

Defines the available permissions independently of individual users and companies.

### Role Permission

Associates roles with permission definitions and any necessary scope restrictions.

### User Permission Assignment

Grants or denies a specific permission to an individual user within an explicitly defined company, branch, or other supported scope.

### Audit Event

Records sensitive changes to users, roles, assignments, and permissions.

Avoid redundant tables if existing GearShift models already represent these relationships. Use database constraints, indexes, foreign keys, and appropriate deletion behavior.

Do not use arbitrary JSON fields as the primary authorization engine when relational models can provide reliable integrity and querying.

## 7. Permission Resolution Rules

Implement one centralized authorization service.

Authorization must consider:

1. Authenticated user.
2. Active company membership.
3. Active branch membership or applicable company-wide scope.
4. Role permissions.
5. Explicit user-specific permission overrides.
6. Requested module, resource, action, and scope.
7. Any additional business rules required by the protected operation.

Define and document precedence rules.

Recommended default behavior:

- No matching grant means deny.
- A role grant authorizes only the scopes covered by that grant.
- An explicit user denial overrides an ordinary role grant within the same scope.
- A narrower branch-specific denial must not silently revoke unrelated permissions in other branches.
- Company-wide access must be explicitly represented and restricted to authorized administrative roles.
- A grant in one company must never authorize access to another company.
- Sensitive administrative permissions must not be inherited accidentally from ordinary operational roles.

If the same user has multiple roles, define how permissions combine and how explicit denials take precedence. Make the result deterministic and testable.

Do not rely on frontend state, request parameters, or user-supplied role names to determine authorization.

## 8. Permission Management Panel

Create an internal administration area in GearShift.

Suggested navigation:

- Company Overview.
- Branches.
- Employees.
- Roles and Permissions.
- Audit History.

### Roles and Permissions Screen

Provide:

- A list of available roles.
- Default and custom role indicators.
- A permission matrix organized by module and action.
- Search and filtering.
- Branch-scope configuration.
- Role creation, editing, duplication, and deletion when safe.
- A way to assign or remove permissions.
- A preview of the affected users and scopes.
- A confirmation step for sensitive changes.

The permission matrix should allow an administrator to inspect which roles can access each module, resource, and action.

### Employee Access Screen

For each employee, display:

- Company membership.
- Assigned roles.
- Assigned branches.
- Accessible modules.
- Effective permissions.
- Explicit exceptions or overrides.
- Membership and assignment status.

Allow authorized administrators to:

- Assign roles.
- Remove roles.
- Add branch access.
- Remove branch access.
- Grant specific additional permissions.
- Deny specific permissions.
- Inspect the resulting effective permissions before saving.

The interface must clearly distinguish inherited permissions from direct user assignments.

### Cross-Branch Permissions

Provide a clear way to grant access to a module in another branch.

Example:

An employee belongs to Branch A and has normal sales permissions there. The company administrator grants that employee read-only inventory access in Branch B.

Expected result:

- Sales operations in Branch A remain unchanged.
- Inventory in Branch B becomes available only for the authorized actions.
- No additional access to cash, employees, reports, or company settings is granted in Branch B.
- Other branches remain inaccessible.

Do not implement cross-branch access as a global employee toggle.

## 9. General and Branch-Level Administration

Support two distinct administrative experiences.

### General Administration

Available only to users with the appropriate company-wide permissions.

Capabilities may include:

- Manage branches.
- View company-wide operational summaries.
- Manage employees across authorized branches.
- Configure roles and permissions.
- Assign cross-branch access.
- Review company-wide audit events.
- Manage company configuration.

### Branch Administration

Available only to users authorized for the relevant branch and administrative actions.

Capabilities may include:

- View branch-specific operational data.
- Manage employees within the permitted branch scope.
- Assign operational roles within authorized limits.
- View branch reports.
- Manage permitted inventory, sales, service orders, and cash operations.

Branch administrators must not be able to grant themselves company-wide privileges, access unauthorized branches, or modify protected system roles.

General administration must not imply unrestricted platform-level superuser access.

## 10. Backend Enforcement

Implement authorization through reusable Django services and permission classes consistent with the existing architecture.

Requirements:

- Enforce permissions on every protected API endpoint.
- Scope database queries by company and authorized branches.
- Validate both object-level and action-level permissions.
- Enforce permissions in services for sensitive business operations, not only at the API boundary.
- Protect background jobs and internal workflows where users' permissions are relevant.
- Prevent unauthorized creation, modification, deletion, exports, and indirect access through related objects.
- Validate company and branch ownership of every referenced object.
- Avoid N+1 permission queries where practical.
- Cache effective permissions only if necessary and with reliable invalidation.
- Revoke access promptly when membership, role, branch scope, or permission assignments change.

Use transactions for permission changes that must remain consistent.

Prevent privilege escalation by ensuring an administrator can grant only permissions within their own authority.

Do not expose global platform-level administrative capabilities to ordinary company administrators.

## 11. Frontend Integration

Implement the system in React and TypeScript using the project's existing conventions.

Create reusable utilities or hooks such as:

- `usePermissions`
- `useCan`
- `useCompanyAccess`
- `useBranchAccess`

Use the actual naming conventions already present in the codebase when appropriate.

The frontend should:

- Hide navigation items and actions that the user cannot access.
- Display only authorized modules and branches.
- Protect internal routes from unauthorized navigation.
- Show a clear unauthorized state when access is denied.
- Support permission loading states without briefly displaying restricted actions.
- Refresh effective permissions after relevant assignments change.
- Avoid duplicating authorization logic across components.

Frontend permission checks are for usability. The backend remains the final authority.

Preserve existing UI patterns, styling, API clients, query management, and error handling.

## 12. Security and Auditability

Record audit events for sensitive actions, including:

- Role creation, modification, and deletion.
- Permission grants and revocations.
- Employee role changes.
- Branch access changes.
- Direct user overrides.
- Changes to general administrative access.
- Failed or suspicious privilege-escalation attempts, where practical.

Audit records should identify the actor, company, affected user or role, relevant branch and permission, action, timestamp, and an appropriate change summary.

Never store passwords, access tokens, or other secrets in audit logs.

Prevent unauthorized users from editing or deleting audit history.

Protect critical administrative roles and prevent an administrator from accidentally removing the last viable company administrator without an explicit recovery or transfer process.

## 13. Database Migrations and Initial Data

Implement Django migrations and an idempotent initialization mechanism for:

- The permission catalog.
- Default company-size templates.
- Default roles.
- Role-to-permission mappings.

Requirements:

- Re-running initialization must not duplicate records.
- Existing companies and users must remain valid.
- Existing authorization behavior must not be weakened during migration.
- Do not assign broad permissions to all existing users as a migration shortcut.
- Provide a safe migration or rollout plan for companies already using GearShift.
- Separate system-provided templates from company-specific customizations.

## 14. Required Tests

Create automated backend tests covering at least:

1. A user cannot access another company's resources.
2. A user cannot access an unassigned branch.
3. A branch manager cannot perform company-wide administration.
4. An employee can access only their assigned modules and actions.
5. A cross-branch grant permits only the specified module and actions.
6. Explicit denials override applicable role grants.
7. Users cannot grant themselves unauthorized permissions.
8. A company administrator cannot modify platform-level permissions.
9. Suspended memberships no longer authorize protected operations.
10. Object identifiers cannot be used to bypass company or branch restrictions.
11. Role and permission changes take effect correctly.
12. Deleting or modifying roles does not leave unintended access.
13. Default templates initialize idempotently.
14. Concurrent permission changes do not corrupt assignments.
15. Existing authentication and business workflows remain functional.

Create frontend tests for permission-based navigation, route protection, permission loading, and administrative controls.

Include regression tests for existing inventory, sales, cash, and service-order workflows wherever these modules exist.

## 15. Implementation Workflow

Proceed in the following order:

### Phase 1: Repository Audit

Inspect the existing:

- User and authentication models.
- Company and branch models.
- Existing roles and permissions.
- Django REST Framework configuration.
- API endpoints and services.
- React routing and navigation.
- Existing business modules.
- Database migrations and tests.

Identify what already exists and what is missing.

### Phase 2: Design

Provide:

- A concise architecture proposal.
- The proposed entity relationships.
- A permission-resolution algorithm.
- A security threat assessment.
- A migration and compatibility strategy.
- A list of affected files.

Prefer the smallest secure design that fits the current architecture.

### Phase 3: Backend

Implement models, migrations, permission catalog, default templates, authorization services, API enforcement, assignment endpoints, and audit events.

### Phase 4: Frontend

Implement the administration screens, permission matrix, employee access management, branch scopes, and effective-permission visualization.

### Phase 5: Verification

Run the existing test suite and new tests. Run formatting, linting, and type checks using the project's existing tools.

Review database queries, transaction handling, tenant isolation, and privilege-escalation paths.

### Phase 6: Delivery Report

Provide:

- Implemented features.
- Database changes and migrations.
- API endpoints added or changed.
- Frontend components added or changed.
- Default roles and permission templates.
- Security guarantees and remaining limitations.
- Tests executed and their results.
- Deployment and migration instructions.
- Any incomplete work or risks.

## 16. Acceptance Criteria

The implementation is complete only when:

- Companies have isolated permission configurations.
- Roles can be scoped by company and branch.
- Modules and actions can be assigned independently.
- Default role templates exist for small, medium, and large companies.
- Administrators can create and customize roles.
- Authorized administrators can grant or revoke permissions for individual users.
- Cross-branch permissions work without exposing unrelated modules.
- General and branch-level administration remain distinct.
- All protected backend operations enforce authorization.
- Effective permissions can be inspected in the interface.
- Permission changes are auditable.
- Automated security tests demonstrate that unauthorized access is denied.
- Existing GearShift functionality remains intact.

**Final instruction:** Implement actual, tested functionality rather than producing only models, mock screens, documentation, or placeholder endpoints. If a requirement conflicts with the current architecture, explain the conflict and propose the smallest safe adaptation before proceeding. Do not introduce unrelated refactors or modify business logic outside the scope of this implementation.